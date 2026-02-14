use image::DynamicImage;

pub fn apply(img: DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
    img.unsharpen(sigma, threshold)
}
