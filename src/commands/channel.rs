use clap::ValueEnum;
use image::{DynamicImage, GrayImage};

#[derive(Clone, ValueEnum)]
pub enum ChannelColor {
    Red,
    Green,
    Blue,
}

pub fn apply(img: DynamicImage, color: ChannelColor) -> DynamicImage {
    let rgb = img.to_rgb8();
    let idx = match color {
        ChannelColor::Red => 0,
        ChannelColor::Green => 1,
        ChannelColor::Blue => 2,
    };
    let gray = GrayImage::from_fn(rgb.width(), rgb.height(), |x, y| {
        image::Luma([rgb.get_pixel(x, y)[idx]])
    });
    DynamicImage::ImageLuma8(gray)
}
