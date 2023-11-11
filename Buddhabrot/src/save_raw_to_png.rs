extern crate image;

use self::image::{GrayImage, ImageFormat, RgbImage};

pub fn save_gray_raw_to_png(raw_gray_data: Vec<u8>, width: u32, height: u32, path: &String) {
    // technically I should think of reflections, but who cares

    let gray_image = GrayImage::from_raw(width, height, raw_gray_data).unwrap();
    let _ = gray_image.save_with_format(path, ImageFormat::Png);
}

pub fn save_rgb_raw_to_png(raw_rgb_data: Vec<u8>, width: u32, height: u32, path: &String) {
    // technically I should think of reflections, but who cares

    let rgb_image = RgbImage::from_raw(width, height, raw_rgb_data).unwrap();
    let _ = rgb_image.save_with_format(path, ImageFormat::Png);
}
