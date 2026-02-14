use image::DynamicImage;

pub fn apply(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}
