// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod image_handler;
pub mod mandelbrot;
pub mod math;
pub mod multithread_demo;

use std::time::Instant;

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

    // image_handler::save_rgb_raw_to_png(
    //     buddhabrot::rgb_buddhabrot::triple_density_map_to_rgb(
    //         &buddhabrot::rgb_buddhabrot::get_buddhabrot_triple_density_map_random_samples_multithreaded(
    //             4092,
    //             default_settings::X_SAMPLE_RANGE,
    //             4092,
    //             default_settings::Y_SAMPLE_RANGE,
    //             1000000000,
    //             2,
    //             &[5000, 500, 50],
    //             5
    //         ),
    //     ),
    //     4092,
    //     4092,
    //     &String::from("outputs/buddhabrot/test/multithread_3/multithread_3.png"),
    // );

    let data =
        image_handler::get_png_as_raw_rgb(&String::from("outputs/buddhabrot/curved/curved.png"));

    image_handler::save_rgb_raw_to_png(
        (0..data.len())
            .into_iter()
            .map(|index| {
                data[4092 * 3 * ((index / 3) % 4092) + 3 * ((index / 3) / 4092) + index % 3]
            })
            .collect(),
        4092,
        4092,
        &String::from("outputs/buddhabrot/transposed/transposed.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}
