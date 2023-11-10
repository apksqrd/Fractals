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

    println!("Elapsed: {:.2?}", start.elapsed());
}
