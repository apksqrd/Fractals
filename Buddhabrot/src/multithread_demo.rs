/// So, I need multithreading to make things fast.
///
/// The first thing I want to multithread is taking in indices to increment then incrementing their corresponding array elements
pub mod density_map_demo {
    use std::thread;

    pub fn singlethread_increment_density_map(
        density_map: &mut Vec<u64>,
        indices_to_increment: &Vec<usize>,
    ) {
        for index in indices_to_increment {
            density_map[*index] += 1;
        }
    }

    /// inspired by mergesort
    ///
    /// I am doubtful that this will work well because there will be too many threads
    ///
    /// After testing, I learned that for many cases, this is actually the best one, so...
    pub mod binary_multithread_recursive {
        use std::{
            sync::Arc,
            thread::{self, JoinHandle},
        };

        /// returns the increased density amount
        ///
        /// To use the increased density amount on an already existing density amount,
        /// just do elementwise addition
        ///
        /// split until threshold is met
        fn splitting_density_map_maker(
            indices_to_increment: Arc<Vec<usize>>,
            min_index: usize,
            max_index: usize,
            max_depth: usize,
            current_depth: usize,
            len_of_density_map: usize,
        ) -> Vec<u64> {
            if current_depth >= max_depth || max_index - min_index <= 1 {
                // solve normally

                let mut density_map: Vec<u64> = vec![0; len_of_density_map];
                for index_to_increment in &indices_to_increment[min_index..max_index] {
                    density_map[*index_to_increment] += 1;
                }

                return density_map;
            }

            // split into two with multi-core then combine the two density maps with elementwise addition
            // I named the two parts left and right

            // index to split by
            let middle_index = (min_index + max_index) / 2;

            let left_thread: JoinHandle<Vec<u64>>;
            {
                // indices_to_increment is a &Vec<usize>
                let indices_to_increment = indices_to_increment.clone();
                let min_index = min_index;
                let max_index = middle_index;
                let max_depth = max_depth;
                let current_depth = current_depth + 1;
                let len_of_density_map = len_of_density_map;
                left_thread = thread::spawn(move || {
                    splitting_density_map_maker(
                        indices_to_increment,
                        min_index,
                        max_index,
                        max_depth,
                        current_depth,
                        len_of_density_map,
                    )
                });
            }

            let right_thread: JoinHandle<Vec<u64>>;
            {
                // indices_to_increment is a &Vec<usize>
                let indices_to_increment = indices_to_increment.clone();
                let min_index = min_index;
                let max_index = middle_index;
                let max_depth = max_depth;
                let current_depth = current_depth + 1;
                let len_of_density_map = len_of_density_map;
                right_thread = thread::spawn(move || {
                    splitting_density_map_maker(
                        indices_to_increment,
                        min_index,
                        max_index,
                        max_depth,
                        current_depth,
                        len_of_density_map,
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

        pub fn helper_function(
            density_map: &mut Vec<u64>,
            indices_to_increment: &Vec<usize>,
            max_depth: usize,
        ) {
            let increased_amounts = splitting_density_map_maker(
                Arc::new(indices_to_increment.clone()),
                0,
                indices_to_increment.len(),
                max_depth,
                0,
                density_map.len(),
            );

            for index in 0..density_map.len() {
                density_map[index] += increased_amounts[index];
            }
        }
    }

    pub fn multithreaded_increment_density_map_normal_chunks(
        density_map: &mut Vec<u64>,
        indices_to_increment: &Vec<usize>,
        chunk_size: usize,
    ) {
        let mut threads: Vec<thread::JoinHandle<Vec<u64>>> = vec![];

        for chunk_of_indices_to_increment in indices_to_increment.chunks(chunk_size) {
            // for ownership and stuff
            let chunk_of_indices_to_increment: Vec<usize> =
                (*chunk_of_indices_to_increment).iter().cloned().collect();
            let density_map_length = density_map.len();

            let thread_to_add = thread::spawn(move || {
                // solve normally

                let mut density_map: Vec<u64> = vec![0; density_map_length];
                for index_to_increment in chunk_of_indices_to_increment {
                    density_map[index_to_increment] += 1;
                }

                return density_map;
            });

            threads.push(thread_to_add);
        }

        let mut results: Vec<Vec<u64>> = vec![];
        for thread_to_wait_for in threads {
            results.push(thread_to_wait_for.join().unwrap());
        }

        for index in 0..density_map.len() {
            for result in &results {
                density_map[index] += (*result)[index];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    // use self::rand::distributions::{Distribution, Uniform};
    use multithread_demo::density_map_demo::*;
    use std::time::Instant;

    #[test]
    fn test() {
        let initial_density_map: Vec<u64> = vec![0; 16000];
        let indices_to_increment: Vec<usize> = vec![0; 100000000];

        // let mut rng = rand::thread_rng();
        // let random_index_generator = Uniform::new(0, initial_density_map.len());
        // for i in 0..indices_to_increment.len() {
        //     indices_to_increment[i] = random_index_generator.sample(&mut rng);
        // }

        // let initial_density_map: Vec<u64> = vec![0; 5];
        // let mut indices_to_increment: Vec<usize> = vec![1, 2, 4, 0, 3, 4, 3, 3, 3, 3];

        println!("Testing single threaded: ");
        let start = Instant::now();
        let mut single_threaded = initial_density_map.clone();
        singlethread_increment_density_map(&mut single_threaded, &indices_to_increment);
        println!("\tElapsed: {:.2?}", start.elapsed());
        println!();

        println!("Testing multi threaded binary recursive: ");
        let start = Instant::now();
        let mut multithreaded_binary_recursive = initial_density_map.clone();
        binary_multithread_recursive::helper_function(
            &mut multithreaded_binary_recursive,
            &indices_to_increment,
            3,
        );
        println!("\tElapsed: {:.2?}", start.elapsed());
        println!();

        println!("Testing multi threaded normal chunks: ");
        let start = Instant::now();
        let mut multithreaded_chunks = initial_density_map.clone();
        multithreaded_increment_density_map_normal_chunks(
            &mut multithreaded_chunks,
            &indices_to_increment,
            (indices_to_increment.len() / 8).max(1), // 8 chunks
        );
        println!("\tElapsed: {:.2?}", start.elapsed());
        println!();

        assert_eq!(single_threaded, multithreaded_binary_recursive,);
        assert_eq!(single_threaded, multithreaded_chunks);
    }
}
