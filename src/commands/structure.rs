use image::{DynamicImage, Rgba};

pub fn apply(img: DynamicImage, amount: i32) -> DynamicImage {
    if amount == 0 {
        return img;
    }

    // Scale blur radius to image dimensions (relative to 1080p baseline)
    let (w, h) = (img.width() as f32, img.height() as f32);
    let scale = w.min(h) / 1080.0;
    let sigma = (20.0 * scale).max(4.0);

    let blurred = img.blur(sigma);

    let strength = amount as f32 / 100.0;

    let orig_buf = img.to_rgba8();
    let blur_buf = blurred.to_rgba8();

    let mut out = orig_buf.clone();

    for (out_px, (orig_px, blur_px)) in out
        .pixels_mut()
        .zip(orig_buf.pixels().zip(blur_buf.pixels()))
    {
        let r = (orig_px[0] as f32 + strength * (orig_px[0] as f32 - blur_px[0] as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        let g = (orig_px[1] as f32 + strength * (orig_px[1] as f32 - blur_px[1] as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = (orig_px[2] as f32 + strength * (orig_px[2] as f32 - blur_px[2] as f32))
            .round()
            .clamp(0.0, 255.0) as u8;
        *out_px = Rgba([r, g, b, orig_px[3]]);
    }

    DynamicImage::ImageRgba8(out)
}
