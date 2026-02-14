use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use image::{DynamicImage, ImageFormat, ImageReader};

mod commands;
mod utils;

use commands::channel::ChannelColor;

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

fn main() {
    let cli = Cli::parse();

    // show-curve doesn't need an input image
    if let Command::ShowCurve { darks, middarks, mids, midhighlights, highlights } = &cli.command {
        let result = commands::show_curve::apply(*darks, *middarks, *mids, *midhighlights, *highlights);
        save_image(&result, cli.output.as_ref());
        return;
    }

    let img = load_image(cli.input.as_ref());

    let result = match cli.command {
        Command::Blur { sigma } => commands::blur::apply(img, sigma),
        Command::Unsharpen { sigma, threshold } => commands::unsharpen::apply(img, sigma, threshold),
        Command::Grayscale => commands::grayscale::apply(img),
        Command::Resize { output_size } => commands::resize::apply(img, output_size),
        Command::Channel { color } => commands::channel::apply(img, color),
        Command::Curve { darks, middarks, mids, midhighlights, highlights } => {
            commands::curve::apply(img, darks, middarks, mids, midhighlights, highlights)
        }
        Command::Color { temperature, tint, vibrance, saturation } => {
            commands::color::apply(img, temperature, tint, vibrance, saturation)
        }
        Command::ColorGrade {
            shadows_hue, shadows_sat, shadows_lum,
            midtones_hue, midtones_sat, midtones_lum,
            highlights_hue, highlights_sat, highlights_lum,
        } => {
            commands::color_grade::apply(
                img,
                shadows_hue, shadows_sat, shadows_lum,
                midtones_hue, midtones_sat, midtones_lum,
                highlights_hue, highlights_sat, highlights_lum,
            )
        }
        Command::Vignette { amount, midpoint, roundness, feather } => {
            commands::vignette::apply(img, amount, midpoint, roundness, feather)
        }
        Command::ShowCurve { .. } => unreachable!(),
    };

    save_image(&result, cli.output.as_ref());
}
