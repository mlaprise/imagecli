use image::DynamicImage;

pub fn apply(img: DynamicImage, output_size: u32) -> DynamicImage {
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
