// TODO: get rid of pub later

pub mod buddhabrot;
pub mod complex;
pub mod mandelbrot;
pub mod math;
pub mod quadratic_recurrence_equation;
pub mod save_raw_to_png;

use mandelbrot::mandelbrot_grid::default_settings;

use crate::save_raw_to_png::save_rgb_raw_to_png;

fn main() {
    use std::time::Instant;
    let start = Instant::now();

    // save_gray_raw_to_png(
    //     buddhabrot::rgb_buddhabrot::triple_density_map_to_rgb(
    //         &buddhabrot::rgb_buddhabrot::get_buddhabrot_triple_density_map_random_samples(
    //             default_settings::X_SAMPLE_SIZE,
    //             default_settings::X_SAMPLE_RANGE,
    //             default_settings::Y_SAMPLE_SIZE,
    //             default_settings::Y_SAMPLE_RANGE,
    //             10000000,
    //             2,
    //             [5000, 500, 50],
    //         ),
    //     ),
    //     default_settings::X_SAMPLE_SIZE as u32,
    //     default_settings::Y_SAMPLE_SIZE as u32,
    //     &String::from("outputs/buddhabrot/test/color_test_1/color_test_1.png"),
    // );
    save_rgb_raw_to_png(
        buddhabrot::rgb_buddhabrot::triple_density_map_to_rgb(
            &buddhabrot::rgb_buddhabrot::get_buddhabrot_triple_density_map_random_samples(
                512,
                default_settings::X_SAMPLE_RANGE,
                512,
                default_settings::Y_SAMPLE_RANGE,
                10000000,
                2,
                [5000, 500, 50],
            ),
        ),
        512,
        512,
        &String::from("outputs/buddhabrot/test/color_test_1/color_test_1.png"),
    );

    println!("Elapsed: {:.2?}", start.elapsed());
}
