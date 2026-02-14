use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use image::{DynamicImage, GrayImage, ImageFormat, ImageReader, Rgb, RgbImage};

#[derive(Parser)]
#[command(name = "imagecli", about = "A simple image processing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Input file path (reads from stdin if omitted)
    #[arg(short, long, global = true)]
    input: Option<PathBuf>,

    /// Output file path (writes PNG to stdout if omitted)
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Apply a Gaussian blur
    Blur {
        /// Blur radius (sigma)
        #[arg(short, long, default_value_t = 2.0)]
        sigma: f32,
    },

    /// Apply an unsharp mask
    Unsharpen {
        /// Blur radius (sigma)
        #[arg(short, long, default_value_t = 2.0)]
        sigma: f32,

        /// Sharpening threshold
        #[arg(short, long, default_value_t = 5)]
        threshold: i32,
    },

    /// Convert to grayscale (black and white)
    Grayscale,

    /// Resize image so the longest side equals output_size (no-op if already smaller)
    Resize {
        /// Target size for the longest side in pixels
        #[arg(short = 's', long)]
        output_size: u32,
    },

    /// Extract a single RGB channel as a grayscale image
    Channel {
        /// Which channel to extract
        #[arg(value_enum)]
        color: ChannelColor,
    },

    /// Tone curve adjustment via a 5-point spline (values on a 0–100 scale)
    Curve {
        /// Dark point adjustment (input=0)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        darks: i32,

        /// Mid-dark point adjustment (input≈25)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        middarks: i32,

        /// Mid point adjustment (input≈50)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        mids: i32,

        /// Mid-highlight point adjustment (input≈75)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        midhighlights: i32,

        /// Highlight point adjustment (input=100)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        highlights: i32,
    },

    /// Debug: render the tone curve as a 256x256 plot (no input image needed)
    ShowCurve {
        /// Dark point adjustment (input=0)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        darks: i32,

        /// Mid-dark point adjustment (input≈25)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        middarks: i32,

        /// Mid point adjustment (input≈50)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        mids: i32,

        /// Mid-highlight point adjustment (input≈75)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        midhighlights: i32,

        /// Highlight point adjustment (input=100)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        highlights: i32,
    },

    /// Adjust color: temperature, tint, vibrance, saturation
    Color {
        /// White balance: -100 (cool/blue) to 100 (warm/orange)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        temperature: i32,

        /// Green-magenta axis: -100 (green) to 100 (magenta)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        tint: i32,

        /// Smart saturation for muted colors: -100 to 100
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        vibrance: i32,

        /// Linear saturation: -100 (grayscale) to 100 (oversaturated)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        saturation: i32,
    },

    /// Color grading: tint shadows, midtones, and highlights independently
    ColorGrade {
        /// Shadows hue (0–360 degrees on color wheel)
        #[arg(long, default_value_t = 0)]
        shadows_hue: u32,
        /// Shadows saturation (0–100, distance from center)
        #[arg(long, default_value_t = 0)]
        shadows_sat: u32,
        /// Shadows luminance shift (-100 to +100)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        shadows_lum: i32,

        /// Midtones hue (0–360 degrees on color wheel)
        #[arg(long, default_value_t = 0)]
        midtones_hue: u32,
        /// Midtones saturation (0–100, distance from center)
        #[arg(long, default_value_t = 0)]
        midtones_sat: u32,
        /// Midtones luminance shift (-100 to +100)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        midtones_lum: i32,

        /// Highlights hue (0–360 degrees on color wheel)
        #[arg(long, default_value_t = 0)]
        highlights_hue: u32,
        /// Highlights saturation (0–100, distance from center)
        #[arg(long, default_value_t = 0)]
        highlights_sat: u32,
        /// Highlights luminance shift (-100 to +100)
        #[arg(long, default_value_t = 0, allow_hyphen_values = true)]
        highlights_lum: i32,
    },

    /// Apply a Lightroom-style vignette effect
    Vignette {
        /// Vignette strength: -100 (darken edges) to 100 (lighten edges)
        #[arg(short, long, default_value_t = -50, allow_hyphen_values = true)]
        amount: i32,

        /// How far from center the effect starts (0–100)
        #[arg(short, long, default_value_t = 50)]
        midpoint: u32,

        /// Shape: -100 (rectangular) to 100 (circular)
        #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
        roundness: i32,

        /// Softness of the transition (0–100)
        #[arg(short, long, default_value_t = 50)]
        feather: u32,
    },
}

#[derive(Clone, ValueEnum)]
enum ChannelColor {
    Red,
    Green,
    Blue,
}

fn load_image(path: Option<&PathBuf>) -> DynamicImage {
    match path {
        Some(p) => ImageReader::open(p)
            .unwrap_or_else(|e| panic!("failed to open {}: {e}", p.display()))
            .decode()
            .unwrap_or_else(|e| panic!("failed to decode {}: {e}", p.display())),
        None => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .expect("failed to read from stdin");
            let reader = ImageReader::new(io::Cursor::new(buf))
                .with_guessed_format()
                .expect("failed to guess image format from stdin");
            reader.decode().expect("failed to decode image from stdin")
        }
    }
}

fn save_image(img: &DynamicImage, path: Option<&PathBuf>) {
    match path {
        Some(p) => img
            .save(p)
            .unwrap_or_else(|e| panic!("failed to save {}: {e}", p.display())),
        None => {
            let mut buf = Vec::new();
            img.write_to(&mut io::Cursor::new(&mut buf), ImageFormat::Png)
                .expect("failed to encode image to PNG");
            io::stdout()
                .write_all(&buf)
                .expect("failed to write to stdout");
        }
    }
}

/// Build a natural cubic spline through a set of (x, y) control points.
/// Returns coefficients (a, b, c, d) for each segment where:
///   S_i(x) = a_i + b_i*(x - x_i) + c_i*(x - x_i)^2 + d_i*(x - x_i)^3
fn cubic_spline_coeffs(xs: &[f64], ys: &[f64]) -> Vec<(f64, f64, f64, f64)> {
    let n = xs.len() - 1;
    let mut h = vec![0.0; n];
    for i in 0..n {
        h[i] = xs[i + 1] - xs[i];
    }

    // Solve tridiagonal system for c coefficients (natural spline: c[0] = c[n] = 0)
    let mut alpha = vec![0.0; n + 1];
    for i in 1..n {
        alpha[i] = (3.0 / h[i]) * (ys[i + 1] - ys[i]) - (3.0 / h[i - 1]) * (ys[i] - ys[i - 1]);
    }

    let mut l = vec![1.0; n + 1];
    let mut mu = vec![0.0; n + 1];
    let mut z = vec![0.0; n + 1];
    for i in 1..n {
        l[i] = 2.0 * (xs[i + 1] - xs[i - 1]) - h[i - 1] * mu[i - 1];
        mu[i] = h[i] / l[i];
        z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
    }

    let mut c = vec![0.0; n + 1];
    let mut b = vec![0.0; n];
    let mut d = vec![0.0; n];
    for j in (0..n).rev() {
        c[j] = z[j] - mu[j] * c[j + 1];
        b[j] = (ys[j + 1] - ys[j]) / h[j] - h[j] * (c[j + 1] + 2.0 * c[j]) / 3.0;
        d[j] = (c[j + 1] - c[j]) / (3.0 * h[j]);
    }

    (0..n).map(|i| (ys[i], b[i], c[i], d[i])).collect()
}

/// Evaluate the cubic spline at a given x value.
fn cubic_spline_eval(xs: &[f64], coeffs: &[(f64, f64, f64, f64)], x: f64) -> f64 {
    // Find the right segment
    let n = coeffs.len();
    let i = if x <= xs[0] {
        0
    } else if x >= xs[n] {
        n - 1
    } else {
        let mut i = 0;
        for j in 1..n {
            if x < xs[j] {
                break;
            }
            i = j;
        }
        i
    };

    let dx = x - xs[i];
    let (a, b, c, d) = coeffs[i];
    a + b * dx + c * dx * dx + d * dx * dx * dx
}

/// Build a 256-entry LUT from a set of spline control points (on 0–100 scale).
fn build_curve_lut(xs: &[f64], ys: &[f64]) -> [u8; 256] {
    let coeffs = cubic_spline_coeffs(xs, ys);
    let mut lut = [0u8; 256];
    for i in 0..256 {
        let x = i as f64 / 255.0 * 100.0; // map 0–255 to 0–100
        let y = cubic_spline_eval(xs, &coeffs, x);
        let out = (y / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8;
        lut[i] = out;
    }
    lut
}

fn hue_to_rgb(hue: f64) -> (f64, f64, f64) {
    let h = (hue % 360.0) / 60.0;
    let x = 1.0 - ((h % 2.0) - 1.0).abs();
    match h as u32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    }
}

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn render_curve_plot(xs: &[f64; 5], ys: &[f64; 5]) -> DynamicImage {
    let size: u32 = 256;
    let margin: u32 = 0;
    let plot = size - 2 * margin;
    let mut img = RgbImage::from_pixel(size, size, Rgb([30, 30, 30]));

    // Draw grid lines at 25%, 50%, 75%
    for pct in [0.25, 0.5, 0.75] {
        let p = margin + (pct * plot as f64) as u32;
        for i in margin..margin + plot {
            img.put_pixel(p, i, Rgb([60, 60, 60]));
            img.put_pixel(i, p, Rgb([60, 60, 60]));
        }
    }

    // Draw identity diagonal (dark gray dashed)
    for i in 0..plot {
        let x = margin + i;
        let y = margin + plot - 1 - i;
        img.put_pixel(x, y, Rgb([80, 80, 80]));
    }

    // Draw the spline curve
    let coeffs = cubic_spline_coeffs(xs, ys);
    for px in 0..plot {
        let input = px as f64 / (plot - 1) as f64 * 100.0;
        let output = cubic_spline_eval(xs, &coeffs, input).clamp(0.0, 100.0);
        let py = ((1.0 - output / 100.0) * (plot - 1) as f64).round() as u32;
        let x = margin + px;
        let y = margin + py;
        // Draw a 2px thick curve
        for dy in 0..2u32 {
            for dx in 0..2u32 {
                let cx = (x + dx).min(size - 1);
                let cy = (y + dy).min(size - 1);
                img.put_pixel(cx, cy, Rgb([255, 255, 255]));
            }
        }
    }

    // Draw control points as small circles
    for i in 0..5 {
        let cpx = margin + (xs[i] / 100.0 * (plot - 1) as f64).round() as u32;
        let cpy = margin + ((1.0 - ys[i] / 100.0) * (plot - 1) as f64).round() as u32;
        for dy in -3i32..=3 {
            for dx in -3i32..=3 {
                if dx * dx + dy * dy <= 9 {
                    let px = (cpx as i32 + dx).clamp(0, size as i32 - 1) as u32;
                    let py = (cpy as i32 + dy).clamp(0, size as i32 - 1) as u32;
                    img.put_pixel(px, py, Rgb([255, 100, 100]));
                }
            }
        }
    }

    DynamicImage::ImageRgb8(img)
}

fn main() {
    let cli = Cli::parse();

    // show-curve doesn't need an input image
    if let Command::ShowCurve { darks, middarks, mids, midhighlights, highlights } = &cli.command {
        let xs = [0.0, 25.0, 50.0, 75.0, 100.0];
        let ys = [
            (0.0 + *darks as f64).clamp(0.0, 100.0),
            (25.0 + *middarks as f64).clamp(0.0, 100.0),
            (50.0 + *mids as f64).clamp(0.0, 100.0),
            (75.0 + *midhighlights as f64).clamp(0.0, 100.0),
            (100.0 + *highlights as f64).clamp(0.0, 100.0),
        ];
        let result = render_curve_plot(&xs, &ys);
        save_image(&result, cli.output.as_ref());
        return;
    }

    let img = load_image(cli.input.as_ref());

    let result = match cli.command {
        Command::Blur { sigma } => img.blur(sigma),
        Command::Unsharpen { sigma, threshold } => img.unsharpen(sigma, threshold),
        Command::Grayscale => img.grayscale(),
        Command::Resize { output_size } => {
            let longest = img.width().max(img.height());
            if longest <= output_size {
                img
            } else {
                let scale = output_size as f64 / longest as f64;
                let nw = (img.width() as f64 * scale).round() as u32;
                let nh = (img.height() as f64 * scale).round() as u32;
                img.resize_exact(nw, nh, image::imageops::FilterType::Lanczos3)
            }
        }
        Command::Curve { darks, middarks, mids, midhighlights, highlights } => {
            let xs = [0.0, 25.0, 50.0, 75.0, 100.0];
            let ys = [
                (0.0 + darks as f64).clamp(0.0, 100.0),
                (25.0 + middarks as f64).clamp(0.0, 100.0),
                (50.0 + mids as f64).clamp(0.0, 100.0),
                (75.0 + midhighlights as f64).clamp(0.0, 100.0),
                (100.0 + highlights as f64).clamp(0.0, 100.0),
            ];
            let lut = build_curve_lut(&xs, &ys);
            let mut rgb = img.to_rgb8();
            for pixel in rgb.pixels_mut() {
                pixel[0] = lut[pixel[0] as usize];
                pixel[1] = lut[pixel[1] as usize];
                pixel[2] = lut[pixel[2] as usize];
            }
            DynamicImage::ImageRgb8(rgb)
        }
        Command::ColorGrade {
            shadows_hue, shadows_sat, shadows_lum,
            midtones_hue, midtones_sat, midtones_lum,
            highlights_hue, highlights_sat, highlights_lum,
        } => {
            let s_sat = shadows_sat.min(100) as f64 / 100.0;
            let m_sat = midtones_sat.min(100) as f64 / 100.0;
            let h_sat = highlights_sat.min(100) as f64 / 100.0;
            let s_lum = shadows_lum.clamp(-100, 100) as f64 / 100.0;
            let m_lum = midtones_lum.clamp(-100, 100) as f64 / 100.0;
            let h_lum = highlights_lum.clamp(-100, 100) as f64 / 100.0;

            let s_tint = hue_to_rgb(shadows_hue as f64);
            let m_tint = hue_to_rgb(midtones_hue as f64);
            let h_tint = hue_to_rgb(highlights_hue as f64);

            // Precompute tint offset directions (signed, -1 to +1 per channel)
            let s_off = ((s_tint.0 - 0.5) * 2.0, (s_tint.1 - 0.5) * 2.0, (s_tint.2 - 0.5) * 2.0);
            let m_off = ((m_tint.0 - 0.5) * 2.0, (m_tint.1 - 0.5) * 2.0, (m_tint.2 - 0.5) * 2.0);
            let h_off = ((h_tint.0 - 0.5) * 2.0, (h_tint.1 - 0.5) * 2.0, (h_tint.2 - 0.5) * 2.0);

            let mut rgb = img.to_rgb8();
            for pixel in rgb.pixels_mut() {
                let r = pixel[0] as f64;
                let g = pixel[1] as f64;
                let b = pixel[2] as f64;
                let lum = (0.2126 * r + 0.7152 * g + 0.0722 * b) / 255.0;

                let shadows_w = 1.0 - smoothstep(0.0, 0.5, lum);
                let highlights_w = smoothstep(0.5, 1.0, lum);
                let midtones_w = 1.0 - shadows_w - highlights_w;

                // Additive color tint from all three ranges
                let cr = s_off.0 * shadows_w * s_sat * 80.0
                       + m_off.0 * midtones_w * m_sat * 80.0
                       + h_off.0 * highlights_w * h_sat * 80.0;
                let cg = s_off.1 * shadows_w * s_sat * 80.0
                       + m_off.1 * midtones_w * m_sat * 80.0
                       + h_off.1 * highlights_w * h_sat * 80.0;
                let cb = s_off.2 * shadows_w * s_sat * 80.0
                       + m_off.2 * midtones_w * m_sat * 80.0
                       + h_off.2 * highlights_w * h_sat * 80.0;

                // Multiplicative luminance shift
                let lum_factor = (1.0 + s_lum * shadows_w * 0.5)
                               * (1.0 + m_lum * midtones_w * 0.5)
                               * (1.0 + h_lum * highlights_w * 0.5);

                pixel[0] = ((r + cr) * lum_factor).round().clamp(0.0, 255.0) as u8;
                pixel[1] = ((g + cg) * lum_factor).round().clamp(0.0, 255.0) as u8;
                pixel[2] = ((b + cb) * lum_factor).round().clamp(0.0, 255.0) as u8;
            }
            DynamicImage::ImageRgb8(rgb)
        }
        Command::Vignette { amount, midpoint, roundness, feather } => {
            let amount = amount.clamp(-100, 100);
            let midpoint = midpoint.clamp(0, 100);
            let roundness = roundness.clamp(-100, 100);
            let feather = feather.clamp(0, 100);

            let mut rgb = img.to_rgb8();
            let w = rgb.width() as f64;
            let h = rgb.height() as f64;
            let longest = w.max(h);

            let t = (roundness as f64 + 100.0) / 200.0; // 0 = rect, 1 = circle
            let radius = midpoint as f64 / 100.0 * 0.75;
            let feather_width = feather as f64 / 100.0 * 0.5;
            let inner = radius;
            let outer = radius + feather_width;
            let amt = amount as f64 / 100.0;

            for y in 0..rgb.height() {
                for x in 0..rgb.width() {
                    let uv_x = (x as f64 / w - 0.5) * (w / longest);
                    let uv_y = (y as f64 / h - 0.5) * (h / longest);

                    let circle_dist = (uv_x * uv_x + uv_y * uv_y).sqrt();
                    let rect_dist = uv_x.abs().max(uv_y.abs());
                    let dist = rect_dist + t * (circle_dist - rect_dist);

                    let strength = smoothstep(inner, outer, dist);

                    let pixel = rgb.get_pixel_mut(x, y);
                    for c in 0..3 {
                        let v = pixel[c] as f64;
                        let new_v = if amt < 0.0 {
                            // Darken: blend toward black
                            v * (1.0 - strength * amt.abs())
                        } else {
                            // Lighten: blend toward white
                            v + (255.0 - v) * strength * amt
                        };
                        pixel[c] = new_v.round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
            DynamicImage::ImageRgb8(rgb)
        }
        Command::Color { temperature, tint, vibrance, saturation } => {
            let temperature = temperature.clamp(-100, 100) as f64;
            let tint = tint.clamp(-100, 100) as f64;
            let vibrance = vibrance.clamp(-100, 100) as f64 / 100.0;
            let saturation = saturation.clamp(-100, 100) as f64 / 100.0;

            // Per-channel multipliers for temperature + tint
            let r_scale = 1.0 + (temperature * 0.15 + tint * 0.05) / 100.0;
            let g_scale = 1.0 + (temperature * 0.05 - tint * 0.15) / 100.0;
            let b_scale = 1.0 - (temperature * 0.15 - tint * 0.05) / 100.0;

            let mut rgb = img.to_rgb8();
            for pixel in rgb.pixels_mut() {
                // Apply temperature + tint
                let r = (pixel[0] as f64 * r_scale).clamp(0.0, 255.0);
                let g = (pixel[1] as f64 * g_scale).clamp(0.0, 255.0);
                let b = (pixel[2] as f64 * b_scale).clamp(0.0, 255.0);

                // Luminance (Rec. 709)
                let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;

                // Pixel saturation for vibrance weighting
                let max_ch = r.max(g).max(b);
                let min_ch = r.min(g).min(b);
                let pixel_sat = if max_ch > 0.0 { (max_ch - min_ch) / max_ch } else { 0.0 };

                // Combined factor: linear saturation * vibrance (inversely weighted by existing saturation)
                let factor = (1.0 + saturation) * (1.0 + vibrance * (1.0 - pixel_sat));

                pixel[0] = (lum + factor * (r - lum)).round().clamp(0.0, 255.0) as u8;
                pixel[1] = (lum + factor * (g - lum)).round().clamp(0.0, 255.0) as u8;
                pixel[2] = (lum + factor * (b - lum)).round().clamp(0.0, 255.0) as u8;
            }
            DynamicImage::ImageRgb8(rgb)
        }
        Command::Channel { color } => {
            let rgb = img.to_rgb8();
            let idx = match color {
                ChannelColor::Red => 0,
                ChannelColor::Green => 1,
                ChannelColor::Blue => 2,
            };
            let gray = GrayImage::from_fn(rgb.width(), rgb.height(), |x, y| {
                image::Luma([rgb.get_pixel(x, y)[idx]])
            });
            DynamicImage::ImageLuma8(gray)
        }
        Command::ShowCurve { .. } => unreachable!(),
    };

    save_image(&result, cli.output.as_ref());
}
