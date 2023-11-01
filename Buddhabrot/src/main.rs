extern crate image;

// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod mandelbrot;
pub mod math;
pub mod quadratic_recurrence_equation;
pub mod save_raw_to_png;

use mandelbrot::mandelbrot_grid::default_settings;
use save_raw_to_png::save_gray_raw_to_png;

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    save_gray_raw_to_png(
        buddhabrot::get_grid_of_buddhabrot_grey(
            default_settings::X_SAMPLE_SIZE,
            default_settings::X_SAMPLE_RANGE,
            default_settings::Y_SAMPLE_SIZE,
            default_settings::Y_SAMPLE_RANGE,
            2,
            100,
        ),
        default_settings::X_SAMPLE_SIZE as u32,
        default_settings::Y_SAMPLE_SIZE as u32,
        &String::from("outputs/buddhabrot/test/3.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}
