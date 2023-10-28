extern crate image;

// TODO: get rid of pub later

pub mod complex;
pub mod mandelbrot;
pub mod math;
pub mod quadratic_recurrence_equation;
pub mod save_raw_to_png;

use mandelbrot::mandelbrot_grid::{default_settings, get_grid_of_rgb_colors};
use save_raw_to_png::save_rgb_raw_to_png;

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    // save_gray_raw_to_png(
    //     mandelbrot::get_grid_of_gray_colors(None, None, None, None, None),
    //     mandelbrot::default_settings::X_SAMPLE_SIZE as u32,
    //     mandelbrot::default_settings::Y_SAMPLE_SIZE as u32,
    //     &String::from("outputs/mandelbrot/test.png"),
    // );

    save_rgb_raw_to_png(
        get_grid_of_rgb_colors(None, None, None, None, None),
        default_settings::X_SAMPLE_SIZE as u32,
        default_settings::Y_SAMPLE_SIZE as u32,
        &String::from("outputs/mandelbrot/test/make_sure_my_refactored_code_isnt_broken.png"),
    );
    // get_grid_of_rgb_colors(None, None, None, None, None);

    println!("Elapsed: {:.2?}", start.elapsed());
}
