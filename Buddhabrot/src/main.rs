extern crate image;

use image::{GrayImage, ImageFormat};

mod complex;
mod mandelbrot;

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    save_gray_raw_to_png(
        mandelbrot::get_grid_of_iterations(None, None, None, None, None),
        mandelbrot::default_settings::X_SAMPLE_SIZE as u32,
        mandelbrot::default_settings::Y_SAMPLE_SIZE as u32,
        &String::from("outputs/outOfBoundsChecker/distSqrdMethod500iter.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}

fn save_gray_raw_to_png(raw_data: Vec<u8>, width: u32, height: u32, path: &String) {
    // technically I should think of reflections, but who cares

    let gray_image = GrayImage::from_raw(width, height, raw_data).unwrap();
    let _ = gray_image.save_with_format(path, ImageFormat::Png);
}