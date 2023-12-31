extern crate image;

use self::image::{io::Reader, GrayImage, ImageFormat, RgbImage};

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

pub fn get_png_as_raw_rgb(path: &String) -> Vec<u8> {
    Reader::open(path).unwrap().decode().unwrap().into_bytes()
}

// put somewhere else?
pub fn get_transpose_rgb_square_grid(original: &Vec<u8>) -> Vec<u8> {
    let len = original.len();
    let side_len = ((len / 3) as f64).sqrt() as usize;

    (0..len)
        .into_iter()
        .map(|index| {
            original
                [side_len * 3 * ((index / 3) % side_len) + 3 * ((index / 3) / side_len) + index % 3]
        })
        .collect()
}
