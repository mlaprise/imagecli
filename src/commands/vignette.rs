use image::DynamicImage;

use crate::utils::smoothstep;

pub fn apply(img: DynamicImage, amount: i32, midpoint: u32, roundness: i32, feather: u32) -> DynamicImage {
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
