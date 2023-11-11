extern crate indicatif;

pub mod mandelbrot_iteration {
    use super::{monoquadratic_symmetric_recurrence_equation, NumberType};
    use complex::Complex;

    fn is_out_of_bounds_currently(current_point: Complex<NumberType>) -> bool {
        // Two main ways of doing it is seeing if total distance is greater than 2,
        // or if it is outside of 4x4 square around origin
        // The speed does not seem to differ much, sometimes square is faster, sometimes tot dist is
        // so, just decide using what looks best.
        // I think the speed difference is because total distance will tall earlier for many cases,
        // so there are less iterations to go through.
        // Also you get more of your money's worth per iteration with total distance

        // total distance
        current_point.distance_sqrd() > 4.
    }

    pub fn get_num_iterations_to_escape(
        initial_point: Complex<NumberType>,
        max_iterations: u64,
    ) -> u64 {
        // NumberType
        let mut point = Complex::new(0., 0.);

        for i in 1..max_iterations {
            point = monoquadratic_symmetric_recurrence_equation(point, initial_point);

            if is_out_of_bounds_currently(point) {
                return i;
            }
        }

        return max_iterations;
    }

    /// returns z_0, z_1, z_2, ...
    /// # Arguments
    /// * `should_remove_out_of_bounds_point` - Only matters when can end early is enabled
    pub fn get_iteration_points(
        initial_point: Complex<NumberType>,
        max_iterations: u64,
        can_end_early_out_of_bounds: bool,
        should_remove_out_of_bounds_point: bool,
    ) -> Vec<Complex<NumberType>> {
        let mut iteration_points = vec![Complex::new(0., 0.)];

        for _ in 1..max_iterations {
            iteration_points.push(monoquadratic_symmetric_recurrence_equation(
                iteration_points[iteration_points.len() - 1],
                initial_point,
            ));

            if can_end_early_out_of_bounds
                && is_out_of_bounds_currently(iteration_points[iteration_points.len() - 1])
            {
                if should_remove_out_of_bounds_point {
                    iteration_points.pop();
                }
                break;
            }
        }

        iteration_points
    }
}

pub mod mandelbrot_grid {
    extern crate indicatif;

    use self::default_settings::MAX_ITERATIONS;
    use self::indicatif::ProgressIterator;
    use super::mandelbrot_iteration::get_num_iterations_to_escape;
    use super::NumberType;
    use complex::Complex;
    use math::remap;

    pub mod default_settings {
        use super::super::NumberType;

        pub const X_SAMPLE_SIZE: usize = 4096; // not sure if usize is correct for this
        pub const X_SAMPLE_RANGE: (NumberType, NumberType) = (-2.1, 2.1);
        pub const Y_SAMPLE_SIZE: usize = 4096;
        pub const Y_SAMPLE_RANGE: (NumberType, NumberType) = (-2.1, 2.1);
        pub const MAX_ITERATIONS: u64 = 500;

        // I am not adding the Optional<> things from now
        // until much later because I am not even going to use it
        // btw i don't actually use this lol
        pub const SHOW_X_PROGRESS_BAR: bool = true;
    }

    pub fn get_grid_of_iterations(
        x_sample_size: Option<usize>,
        x_sample_range: Option<(NumberType, NumberType)>,
        y_sample_size: Option<usize>,
        y_sample_range: Option<(NumberType, NumberType)>,
        max_iterations: Option<u64>,
    ) -> Vec<u64> {
        let x_sample_size = x_sample_size.unwrap_or(default_settings::X_SAMPLE_SIZE);
        let x_sample_range = x_sample_range.unwrap_or(default_settings::X_SAMPLE_RANGE);
        let y_sample_size = y_sample_size.unwrap_or(default_settings::Y_SAMPLE_SIZE);
        let y_sample_range = y_sample_range.unwrap_or(default_settings::Y_SAMPLE_RANGE);
        let max_iterations = max_iterations.unwrap_or(default_settings::MAX_ITERATIONS);

        let mut raw_data = vec![0; x_sample_size * y_sample_size];

        // let x_iterator: iterator_optional_progress!(SHOW_PROGRESS_BAR, x_sample_size);
        let x_iterator: Box<dyn Iterator<Item = usize>> = if default_settings::SHOW_X_PROGRESS_BAR {
            Box::new((0..x_sample_size).progress())
        } else {
            Box::new(0..x_sample_size)
        };

        for x_index in x_iterator {
            for y_index in 0..y_sample_size {
                let x_sample = remap(
                    x_index as NumberType,
                    // do I add one or not?
                    (0., (x_sample_size) as NumberType),
                    x_sample_range,
                );
                let y_sample = remap(
                    y_index as NumberType,
                    (0., (y_sample_size) as NumberType),
                    y_sample_range,
                );

                let point = Complex::new(x_sample, y_sample);

                let iterations = get_num_iterations_to_escape(point, max_iterations);

                raw_data[x_index + x_sample_size * y_index] = iterations;
            }
        }

        raw_data
    }

    fn num_iterations_to_gray_color(num_iterations: u64, max_iterations: u64) -> u8 {
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
            (256.
                * (num_iterations_float
                    / (1. + num_iterations_float * num_iterations_float).sqrt()))
            .floor(),
        ) as u8
    }

    pub fn get_grid_of_gray_colors(
        x_sample_size: Option<usize>,
        x_sample_range: Option<(NumberType, NumberType)>,
        y_sample_size: Option<usize>,
        y_sample_range: Option<(NumberType, NumberType)>,
        max_iterations: Option<u64>,
    ) -> Vec<u8> {
        let grid_of_iterations = get_grid_of_iterations(
            x_sample_size,
            x_sample_range,
            y_sample_size,
            y_sample_range,
            max_iterations,
        );
        grid_of_iterations
            .iter()
            .map(|num_iterations| {
                num_iterations_to_gray_color(
                    *num_iterations,
                    max_iterations.unwrap_or(MAX_ITERATIONS),
                )
            })
            .collect()
    }

    fn num_iterations_to_rgb_color(num_iterations: u64, max_iterations: u64) -> [u8; 3] {
        if num_iterations == max_iterations {
            return [0, 0, 0];
        }

        // from my old python code

        let num_iterations_float = num_iterations as NumberType; // can just be f64, but who cares
        let x = remap(
            // to the fourth power
            (1. - 1. / (num_iterations_float + 1.))
                * (1. - 1. / (num_iterations_float + 1.))
                * (1. - 1. / (num_iterations_float + 1.))
                * (1. - 1. / (num_iterations_float + 1.)),
            (0., 1.),
            (0., 255.),
        );
        [
            f64::from((255. - x / 2.).floor()) as u8,
            f64::from((x).floor()) as u8,
            f64::from((x).floor()) as u8,
        ]
    }

    pub fn get_grid_of_rgb_colors(
        x_sample_size: Option<usize>,
        x_sample_range: Option<(NumberType, NumberType)>,
        y_sample_size: Option<usize>,
        y_sample_range: Option<(NumberType, NumberType)>,
        max_iterations: Option<u64>,
    ) -> Vec<u8> {
        let grid_of_iterations = get_grid_of_iterations(
            x_sample_size,
            x_sample_range,
            y_sample_size,
            y_sample_range,
            max_iterations,
        );
        grid_of_iterations
            .iter()
            .flat_map(|num_iterations| {
                num_iterations_to_rgb_color(
                    *num_iterations,
                    max_iterations.unwrap_or(MAX_ITERATIONS),
                )
            })
            .collect()
    }
}

use complex::Complex;

pub type NumberType = f64; // maybe I should've used a macro

/**
 * FIXME: help, i don't know a good name for this.
 * The name I want is for the equation: f(z) = z^2 + c or z_{n+1} = z_n^2 + c.
 * I don't actually care if it is recursive or not
 *
 * The quadratic recurrence equation has a coefficient for the 2nd and 1st terms:
 * f(x) = az^2 + bz + c
 * but I just want them to always be 1 and 0, respectively.
 * abcz can all be complex (I think)
 *
 * I keep switching between putting this in the general math, somewhere in mandelbrot, or giving this it's own file.
 */
pub fn monoquadratic_symmetric_recurrence_equation(
    z: Complex<NumberType>,
    c: Complex<NumberType>,
) -> Complex<NumberType> {
    z.square() + c
}
