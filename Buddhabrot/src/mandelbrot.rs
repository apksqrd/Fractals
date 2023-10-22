use complex::Complex;

use self::default_settings::NumberType;

pub mod default_settings {
    pub type NumberType = f64; // maybe I should've used a macro

    pub const X_SAMPLE_SIZE: usize = 4096; // not sure if usize is correct for this
    pub const X_SAMPLE_RANGE: (NumberType, NumberType) = (-2., 1.);
    pub const Y_SAMPLE_SIZE: usize = 4096;
    pub const Y_SAMPLE_RANGE: (NumberType, NumberType) = (-1.5, 1.5);
    pub const MAX_ITERATIONS: u32 = 500;
}

// I could (should) put this somewhere else but nrn
pub fn remap(
    value: NumberType,
    initial_range: (NumberType, NumberType),
    new_range: (NumberType, NumberType),
) -> NumberType {
    new_range.0
        + (new_range.1 - new_range.0) * (value - initial_range.0)
            / (initial_range.1 - initial_range.0)
}

pub fn is_out_of_bounds_currently(current_point: Complex<NumberType>) -> bool {
    // Two main ways of doing it is seeing if total distance is greater than 2,
    // or if it is outside of 4x4 square around origin
    // The speed does not seem to differ much, sometimes square is faster, sometimes tot dist is
    // so, just decide using what looks best.

    // total distance
    current_point.distance_sqrd() > 2.

    // // square
    // !(-2. <= current_point.real_component
    //     && current_point.real_component <= 2.
    //     && -2. <= current_point.imaginary_component
    //     && current_point.imaginary_component <= 2.)
}

pub fn get_num_iterations_to_escape(
    initial_point: Complex<NumberType>,
    max_iterations: Option<u32>,
) -> u32 {
    // NumberType
    let max_iterations = max_iterations.unwrap_or(default_settings::MAX_ITERATIONS);
    let mut point = initial_point;

    for i in 0..max_iterations {
        if is_out_of_bounds_currently(point) {
            return i;
        }

        point = point * point + initial_point;
    }

    return max_iterations;
}

pub fn num_iterations_to_color(num_iterations: u32, max_iterations: u32) -> u8 {
    if num_iterations == max_iterations {
        return 0;
    }
    // remap(
    //     num_iterations as NumberType,
    //     (0., (max_iterations - 1) as NumberType),
    //     (0., 256.),
    // )
    // .floor() as u8

    // sigmoidal function
    let num_iterations_float = num_iterations as NumberType; // can just be f64, but who cares
    f64::from(
        (256. * (num_iterations_float / (1. + num_iterations_float * num_iterations_float).sqrt()))
            .floor(),
    ) as u8
}

pub fn get_grid_of_iterations(
    x_sample_size: Option<usize>,
    x_sample_range: Option<(NumberType, NumberType)>,
    y_sample_size: Option<usize>,
    y_sample_range: Option<(NumberType, NumberType)>,
    max_iterations: Option<u32>,
) -> Vec<u8> {
    let x_sample_size = x_sample_size.unwrap_or(default_settings::X_SAMPLE_SIZE);
    let x_sample_range = x_sample_range.unwrap_or(default_settings::X_SAMPLE_RANGE);
    let y_sample_size = y_sample_size.unwrap_or(default_settings::Y_SAMPLE_SIZE);
    let y_sample_range = y_sample_range.unwrap_or(default_settings::Y_SAMPLE_RANGE);
    let max_iterations = max_iterations.unwrap_or(default_settings::MAX_ITERATIONS);

    let mut raw_data = vec![0; x_sample_size * y_sample_size];

    for x_index in 0..x_sample_size {
        for y_index in 0..y_sample_size {
            let x_sample = remap(
                x_index as NumberType,
                (0., (x_sample_size + 1) as NumberType),
                x_sample_range,
            );
            let y_sample = remap(
                y_index as NumberType,
                (0., (y_sample_size + 1) as NumberType),
                y_sample_range,
            );

            let point = Complex::new(x_sample, y_sample);

            let iterations = get_num_iterations_to_escape(point, Some(max_iterations));

            raw_data[(x_index + x_sample_size * y_index) as usize] =
                num_iterations_to_color(iterations, max_iterations);
        }
    }

    raw_data
}
