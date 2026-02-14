use image::DynamicImage;

pub fn apply(img: DynamicImage, sigma: f32) -> DynamicImage {
    img.blur(sigma)
}
