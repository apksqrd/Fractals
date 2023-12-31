// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod image_handler;
pub mod mandelbrot;
pub mod math;
pub mod multithread_demo;

use std::time::Instant;

use crate::mandelbrot::mandelbrot_grid;

fn main() {
    let start = Instant::now();

    image_handler::save_rgb_raw_to_png(
        image_handler::get_transpose_rgb_square_grid(&mandelbrot_grid::get_grid_of_rgb_colors(
            Some(4092),
            Some((-2.1, 2.1)),
            Some(4092),
            Some((-2.1, 2.1)),
            Some(10000),
        )),
        4092,
        4092,
        &String::from("outputs/mandelbrot/differentRange/differentRange.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}
