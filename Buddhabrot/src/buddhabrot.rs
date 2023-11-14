extern crate indicatif;
extern crate rand;

mod increment_density_map {
    use complex::Complex;
    use mandelbrot::mandelbrot_iteration;
    use mandelbrot::NumberType;
    use math::remap;

    fn increment_density_map_integers(
        density_map: &mut Vec<u64>,
        x_index: usize,
        y_index: usize,
        x_sample_size: usize,
        y_sample_size: usize,
    ) {
        if x_index >= x_sample_size || y_index >= y_sample_size {
            panic!("Out of range. x_index = {:?}, y_index = {:?}, x_sample_size = {:?}, y_sample_size = {:?}", x_index, y_index, x_sample_size, y_sample_size);
        }
        density_map[y_index * x_sample_size + x_index] += 1;
    }

    // fn remap_data_to_index()

    fn remapped_increment_density_map(
        density_map: &mut Vec<u64>,
        x_sample: NumberType,
        y_sample: NumberType,
        x_sample_size: usize,
        y_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_range: (NumberType, NumberType),
    ) {
        let x_remapped = remap(x_sample, x_sample_range, (0., x_sample_size as NumberType));
        let y_remapped = remap(y_sample, y_sample_range, (0., y_sample_size as NumberType));

        let x_index = x_remapped.floor() as usize;
        let y_index = y_remapped.floor() as usize;

        increment_density_map_integers(density_map, x_index, y_index, x_sample_size, y_sample_size)
    }

    pub fn increment_density_map_complex(
        density_map: &mut Vec<u64>,
        point_to_increase: Complex<NumberType>,
        x_sample_size: usize,
        y_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_range: (NumberType, NumberType),
    ) {
        remapped_increment_density_map(
            density_map,
            point_to_increase.real_component,
            point_to_increase.imaginary_component,
            x_sample_size,
            y_sample_size,
            x_sample_range,
            y_sample_range,
        )
    }

    pub fn increment_density_map_with_buddhabrot_from_point(
        density_map: &mut Vec<u64>,
        initial_point: Complex<NumberType>,
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        start_iteration: u64,
        max_iterations: u64,
    ) {
        let iteration_points =
            mandelbrot_iteration::get_iteration_points(initial_point, max_iterations, true, true);

        if iteration_points.len() == max_iterations as usize {
            // the point is in the mandelbrot set
            return;
        }

        if iteration_points.len() < start_iteration as usize {
            // left too early that it isn't inside, also will cause an error if I don't return
            return;
        }

        for point in &iteration_points[start_iteration as usize..] {
            increment_density_map_complex(
                density_map,
                *point,
                x_sample_size,
                y_sample_size,
                x_sample_range,
                y_sample_range,
            )
        }
    }
}

pub mod buddhabrot_density_map {
    use std::thread::{self, JoinHandle};

    use super::increment_density_map::increment_density_map_with_buddhabrot_from_point;
    use super::indicatif::ProgressIterator;
    use super::rand::distributions::Uniform;
    use super::rand::{thread_rng, Rng};
    use complex::Complex;
    use mandelbrot::{self, NumberType};
    use math::remap;

    pub fn get_buddhabrot_density_map(
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        start_iteration: u64,
        max_iterations: u64,
    ) -> Vec<u64> {
        let mut density_map = vec![0; x_sample_size * y_sample_size];

        let x_iterator: Box<dyn Iterator<Item = usize>> =
            if mandelbrot::mandelbrot_grid::default_settings::SHOW_X_PROGRESS_BAR {
                Box::new((0..x_sample_size).progress())
            } else {
                Box::new(0..x_sample_size)
            };

        for x_index in x_iterator {
            for y_index in 0..y_sample_size {
                let x_sample = remap(
                    x_index as NumberType,
                    (0., (x_sample_size) as NumberType),
                    x_sample_range,
                );
                let y_sample = remap(
                    y_index as NumberType,
                    (0., (y_sample_size) as NumberType),
                    y_sample_range,
                );

                let point = Complex::new(x_sample, y_sample);

                increment_density_map_with_buddhabrot_from_point(
                    &mut density_map,
                    point,
                    x_sample_size,
                    x_sample_range,
                    y_sample_size,
                    y_sample_range,
                    start_iteration,
                    max_iterations,
                )
            }
        }

        density_map
    }

    pub fn get_buddhabrot_density_map_random_samples(
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        num_samples: u64,
        start_iteration: u64,
        max_iterations: u64,
    ) -> Vec<u64> {
        let mut density_map = vec![0; x_sample_size * y_sample_size];

        let samples_iterator: Box<dyn Iterator<Item = usize>> =
            if mandelbrot::mandelbrot_grid::default_settings::SHOW_X_PROGRESS_BAR {
                Box::new((0..num_samples as usize).progress())
            } else {
                Box::new(0..num_samples as usize)
            };

        let mut rng = thread_rng();
        let random_float_generator = Uniform::new(-2., 2.);

        for _ in samples_iterator {
            let x_sample = rng.sample(random_float_generator);
            let y_sample = rng.sample(random_float_generator);

            let point = Complex::new(x_sample, y_sample);

            increment_density_map_with_buddhabrot_from_point(
                &mut density_map,
                point,
                x_sample_size,
                x_sample_range,
                y_sample_size,
                y_sample_range,
                start_iteration,
                max_iterations,
            )
        }

        density_map
    }

    pub fn get_buddhabrot_density_map_random_samples_multithread(
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        num_samples: u64,
        start_iteration: u64,
        max_iterations: u64,
        num_thread_split_layers: u64,
    ) -> Vec<u64> {
        if num_thread_split_layers <= 1 {
            // solve normally
            return get_buddhabrot_density_map_random_samples(
                x_sample_size,
                x_sample_range,
                y_sample_size,
                y_sample_range,
                num_samples,
                start_iteration,
                max_iterations,
            );
        }

        let left_thread: JoinHandle<Vec<u64>>;
        {
            // left
            let x_sample_size = x_sample_size;
            let x_sample_range = x_sample_range;
            let y_sample_size = y_sample_size;
            let y_sample_range = y_sample_range;
            let num_samples = num_samples;
            let start_iteration = start_iteration;
            let max_iterations = max_iterations;
            let num_thread_split_layers = num_thread_split_layers;

            left_thread = thread::spawn(move || {
                get_buddhabrot_density_map_random_samples_multithread(
                    x_sample_size,
                    x_sample_range,
                    y_sample_size,
                    y_sample_range,
                    num_samples / 2,
                    start_iteration,
                    max_iterations,
                    num_thread_split_layers - 1,
                )
            });
        }

        let right_thread: JoinHandle<Vec<u64>>;
        {
            // right
            let x_sample_size = x_sample_size;
            let x_sample_range = x_sample_range;
            let y_sample_size = y_sample_size;
            let y_sample_range = y_sample_range;
            let num_samples = num_samples;
            let start_iteration = start_iteration;
            let max_iterations = max_iterations;
            let num_thread_split_layers = num_thread_split_layers;

            right_thread = thread::spawn(move || {
                get_buddhabrot_density_map_random_samples_multithread(
                    x_sample_size,
                    x_sample_range,
                    y_sample_size,
                    y_sample_range,
                    num_samples - (num_samples / 2),
                    start_iteration,
                    max_iterations,
                    num_thread_split_layers - 1,
                )
            });
        }

        let left_result = left_thread.join().unwrap();
        let right_result = right_thread.join().unwrap();

        left_result
            .iter()
            .zip(&right_result)
            .map(|(a, b)| *a + *b)
            .collect()
    }
}

pub mod gray_buddhabrot {
    use mandelbrot::NumberType;

    fn density_to_gray(density: u64, max_density: u64) -> u8 {
        let density_float = density as NumberType; // can just be f64, but who cares
        let max_density_float = max_density as NumberType; // can just be f64, but who cares

        // f64::from((256. * (density_float / (1. + density_float * density_float).sqrt())).floor()) as u8

        // remap(density_float, (0., max_density as NumberType), (0., 255.)).floor() as u8

        // f64::from(
        //     (255. * (density_float * density_float) / (max_density_float * max_density_float)).ceil(),
        // ) as u8

        let pre_squared = (density_float - max_density_float) / max_density_float;
        f64::from((255. * (1. - pre_squared * pre_squared)).ceil()) as u8
    }

    pub fn density_map_to_gray(density_map: &Vec<u64>) -> Vec<u8> {
        let max_density = *density_map.iter().max().unwrap();
        density_map
            .iter()
            .map(|num_iterations| density_to_gray(*num_iterations, max_density))
            .collect()
    }
}

pub mod rgb_buddhabrot {
    use std::cmp::min;
    use std::thread::{self, JoinHandle};

    use super::gray_buddhabrot::density_map_to_gray;
    use super::increment_density_map::increment_density_map_complex;
    use super::indicatif::ProgressIterator;
    use super::rand::distributions::Uniform;
    use super::rand::{thread_rng, Rng};
    use complex::Complex;
    use mandelbrot;
    use mandelbrot::{mandelbrot_iteration, NumberType};

    fn increment_triple_density_map_with_buddhabrot_from_point(
        triple_density_map: &mut [Vec<u64>; 3],
        initial_point: Complex<NumberType>,
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        start_iteration: u64,
        max_iterations_triple: &[u64; 3],
    ) {
        let largest_max_iteration = *max_iterations_triple.iter().max().unwrap();

        let iteration_points = mandelbrot_iteration::get_iteration_points(
            initial_point,
            largest_max_iteration,
            true,
            true,
        );

        if iteration_points.len() == largest_max_iteration as usize {
            // the point is in the mandelbrot set
            return;
        }

        if iteration_points.len() < start_iteration as usize {
            // left too early that it isn't inside, also will cause an error if I don't return
            return;
        }

        for i in 0..3 {
            for point in &iteration_points[start_iteration as usize
                ..(min(max_iterations_triple[i] as usize, iteration_points.len()))]
            {
                increment_density_map_complex(
                    &mut triple_density_map[i],
                    *point,
                    x_sample_size,
                    y_sample_size,
                    x_sample_range,
                    y_sample_range,
                )
            }
        }
    }

    /// The reason why this exists is to save rendering time
    pub fn get_buddhabrot_triple_density_map_random_samples(
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        num_samples: u64,
        start_iteration: u64,
        max_iterations_triple: &[u64; 3],
    ) -> [Vec<u64>; 3] {
        let mut triple_density_map = [
            vec![0 as u64; x_sample_size * y_sample_size],
            vec![0 as u64; x_sample_size * y_sample_size],
            vec![0 as u64; x_sample_size * y_sample_size],
        ];

        let samples_iterator: Box<dyn Iterator<Item = usize>> =
            if mandelbrot::mandelbrot_grid::default_settings::SHOW_X_PROGRESS_BAR {
                Box::new((0..num_samples as usize).progress())
            } else {
                Box::new(0..num_samples as usize)
            };

        let mut rng = thread_rng();
        let random_float_generator = Uniform::new(-2., 2.);

        for _ in samples_iterator {
            let x_sample = rng.sample(random_float_generator);
            let y_sample = rng.sample(random_float_generator);

            let point = Complex::new(x_sample, y_sample);

            increment_triple_density_map_with_buddhabrot_from_point(
                &mut triple_density_map,
                point,
                x_sample_size,
                x_sample_range,
                y_sample_size,
                y_sample_range,
                start_iteration,
                max_iterations_triple,
            )
        }

        triple_density_map
    }

    pub fn get_buddhabrot_triple_density_map_random_samples_multithreaded(
        x_sample_size: usize,
        x_sample_range: (NumberType, NumberType),
        y_sample_size: usize,
        y_sample_range: (NumberType, NumberType),
        num_samples: u64,
        start_iteration: u64,
        max_iterations_triple: &[u64; 3],
        num_thread_split_layers: u64,
    ) -> [Vec<u64>; 3] {
        if num_thread_split_layers <= 1 {
            // solve normally
            return get_buddhabrot_triple_density_map_random_samples(
                x_sample_size,
                x_sample_range,
                y_sample_size,
                y_sample_range,
                num_samples,
                start_iteration,
                max_iterations_triple,
            );
        }

        let left_thread: JoinHandle<[Vec<u64>; 3]>;
        {
            // left
            let x_sample_size = x_sample_size;
            let x_sample_range = x_sample_range;
            let y_sample_size = y_sample_size;
            let y_sample_range = y_sample_range;
            let num_samples = num_samples;
            let start_iteration = start_iteration;
            let max_iterations_triple = [
                max_iterations_triple[0],
                max_iterations_triple[1],
                max_iterations_triple[2],
            ];
            let num_thread_split_layers = num_thread_split_layers;

            left_thread = thread::spawn(move || {
                get_buddhabrot_triple_density_map_random_samples_multithreaded(
                    x_sample_size,
                    x_sample_range,
                    y_sample_size,
                    y_sample_range,
                    num_samples / 2,
                    start_iteration,
                    &max_iterations_triple,
                    num_thread_split_layers - 1,
                )
            });
        }

        let right_thread: JoinHandle<[Vec<u64>; 3]>;
        {
            // right
            let x_sample_size = x_sample_size;
            let x_sample_range = x_sample_range;
            let y_sample_size = y_sample_size;
            let y_sample_range = y_sample_range;
            let num_samples = num_samples;
            let start_iteration = start_iteration;
            let max_iterations_triple = [
                max_iterations_triple[0],
                max_iterations_triple[1],
                max_iterations_triple[2],
            ];
            let num_thread_split_layers = num_thread_split_layers;

            right_thread = thread::spawn(move || {
                get_buddhabrot_triple_density_map_random_samples_multithreaded(
                    x_sample_size,
                    x_sample_range,
                    y_sample_size,
                    y_sample_range,
                    num_samples / 2,
                    start_iteration,
                    &max_iterations_triple,
                    num_thread_split_layers - 1,
                )
            });
        }

        let left_result = left_thread.join().unwrap();
        let right_result = right_thread.join().unwrap();

        [
            left_result[0]
                .iter()
                .zip(&right_result[0])
                .map(|(a, b)| *a + *b)
                .collect(),
            left_result[1]
                .iter()
                .zip(&right_result[1])
                .map(|(a, b)| *a + *b)
                .collect(),
            left_result[2]
                .iter()
                .zip(&right_result[2])
                .map(|(a, b)| *a + *b)
                .collect(),
        ]
    }

    pub fn triple_density_map_to_rgb(triple_density_map: &[Vec<u64>; 3]) -> Vec<u8> {
        let mut triple_grays: [Vec<u8>; 3] = [vec![], vec![], vec![]];
        for i in 0..3 {
            triple_grays[i] = density_map_to_gray(&triple_density_map[i]);
        }

        let mut rgb_grid: Vec<u8> = vec![0; 3 * triple_grays[0].len()];
        for i in 0..triple_grays[0].len() {
            for j in 0..3 {
                rgb_grid[3 * i + j] = triple_grays[j][i];
            }
        }

        rgb_grid
    }
}
