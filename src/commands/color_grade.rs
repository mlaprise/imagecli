use image::DynamicImage;

use crate::utils::smoothstep;

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

pub fn apply(
    img: DynamicImage,
    shadows_hue: u32, shadows_sat: u32, shadows_lum: i32,
    midtones_hue: u32, midtones_sat: u32, midtones_lum: i32,
    highlights_hue: u32, highlights_sat: u32, highlights_lum: i32,
) -> DynamicImage {
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
