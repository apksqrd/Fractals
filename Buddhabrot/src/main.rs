// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod mandelbrot;
pub mod math;
pub mod multithread_demo;
pub mod save_raw_to_png;

use std::time::Instant;

use crate::mandelbrot::mandelbrot_grid::default_settings;

fn main() {
    let start = Instant::now();

    // save_raw_to_png::save_gray_raw_to_png(
    //     buddhabrot::gray_buddhabrot::density_map_to_gray(
    //         &buddhabrot::buddhabrot_density_map::get_buddhabrot_density_map_random_samples_multithread(
    //             default_settings::X_SAMPLE_SIZE,
    //             default_settings::X_SAMPLE_RANGE,
    //             default_settings::Y_SAMPLE_SIZE,
    //             default_settings::Y_SAMPLE_RANGE,
    //             100000000,
    //             2,
    //             default_settings::MAX_ITERATIONS,
    //             4
    //         ),
    //     ),
    //     default_settings::X_SAMPLE_SIZE as u32,
    //     default_settings::Y_SAMPLE_SIZE as u32,
    //     &String::from("outputs/buddhabrot/test/multithread_1/multithread_1.png"),
    // );

    save_raw_to_png::save_rgb_raw_to_png(
        buddhabrot::rgb_buddhabrot::triple_density_map_to_rgb(
            &buddhabrot::rgb_buddhabrot::get_buddhabrot_triple_density_map_random_samples_multithreaded(
                4092,
                default_settings::X_SAMPLE_RANGE,
                4092,
                default_settings::Y_SAMPLE_RANGE,
                1000000000,
                2,
                &[5000, 500, 50],
                5
            ),
        ),
        4092,
        4092,
        &String::from("outputs/buddhabrot/test/multithread_3/multithread_3.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}
