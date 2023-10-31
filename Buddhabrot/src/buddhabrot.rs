extern crate indicatif;

use std::thread::yield_now;

use self::indicatif::ProgressIterator;

use crate::{
    complex::{self, Complex},
    mandelbrot::{self, mandelbrot_iteration, NumberType},
    math::remap,
};

fn increment_density_map_integers(
    density_map: &mut Vec<u32>,
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
    density_map: &mut Vec<u32>,
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

fn increment_density_map_complex(
    density_map: &mut Vec<u32>,
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

fn increment_density_map_with_buddhabrot_from_point(
    density_map: &mut Vec<u32>,
    initial_point: Complex<NumberType>,
    x_sample_size: usize,
    x_sample_range: (NumberType, NumberType),
    y_sample_size: usize,
    y_sample_range: (NumberType, NumberType),
    start_iteration: u32,
    max_iterations: u32,
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

pub fn get_grid_of_buddhabrot_density_map(
    x_sample_size: usize,
    x_sample_range: (NumberType, NumberType),
    y_sample_size: usize,
    y_sample_range: (NumberType, NumberType),
    start_iteration: u32,
    max_iterations: u32,
) -> Vec<u32> {
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

fn density_to_grey(density: u32) -> u8 {
    let density_float = density as NumberType; // can just be f64, but who cares

    f64::from((256. * (density_float / (1. + density_float * density_float).sqrt())).floor()) as u8
}

pub fn get_grid_of_buddhabrot_grey(
    x_sample_size: usize,
    x_sample_range: (NumberType, NumberType),
    y_sample_size: usize,
    y_sample_range: (NumberType, NumberType),
    start_iteration: u32,
    max_iterations: u32,
) -> Vec<u8> {
    let density_map = get_grid_of_buddhabrot_density_map(
        x_sample_size,
        x_sample_range,
        y_sample_size,
        y_sample_range,
        start_iteration,
        max_iterations,
    );
    density_map
        .iter()
        .map(|num_iterations| density_to_grey(*num_iterations))
        .collect()
}
