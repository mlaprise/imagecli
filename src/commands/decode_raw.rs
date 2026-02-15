use std::path::PathBuf;

use image::DynamicImage;

pub fn apply(path: &PathBuf) -> DynamicImage {
    let raw_image = rawler::decode_file(path.to_str().unwrap())
        .unwrap_or_else(|e| panic!("failed to decode RAW {}: {e}", path.display()));
    let develop = rawler::imgop::develop::RawDevelop::default();
    let intermediate = develop
        .develop_intermediate(&raw_image)
        .unwrap_or_else(|e| panic!("failed to develop RAW {}: {e}", path.display()));
    intermediate
        .to_dynamic_image()
        .unwrap_or_else(|| panic!("failed to convert RAW to image: {}", path.display()))
}
