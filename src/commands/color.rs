use image::DynamicImage;

pub fn apply(img: DynamicImage, temperature: i32, tint: i32, vibrance: i32, saturation: i32) -> DynamicImage {
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
