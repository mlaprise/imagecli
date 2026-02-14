use image::{DynamicImage, Rgb, RgbImage};

use super::curve::{cubic_spline_coeffs, cubic_spline_eval};

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

pub fn apply(darks: i32, middarks: i32, mids: i32, midhighlights: i32, highlights: i32) -> DynamicImage {
    let xs = [0.0, 25.0, 50.0, 75.0, 100.0];
    let ys = [
        (0.0 + darks as f64).clamp(0.0, 100.0),
        (25.0 + middarks as f64).clamp(0.0, 100.0),
        (50.0 + mids as f64).clamp(0.0, 100.0),
        (75.0 + midhighlights as f64).clamp(0.0, 100.0),
        (100.0 + highlights as f64).clamp(0.0, 100.0),
    ];
    render_curve_plot(&xs, &ys)
}
