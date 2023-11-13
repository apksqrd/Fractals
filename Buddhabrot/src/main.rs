// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod mandelbrot;
pub mod math;
pub mod multithread_demo;
pub mod quadratic_recurrence_equation;
pub mod save_raw_to_png;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Elapsed: {:.2?}", start.elapsed());
}
