use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

fn generate_matrix(size: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| (0..size).map(|_| rng.gen::<f64>()).collect())
        .collect()
}

fn parallel_matrix_multiplication(
    a: &Vec<Vec<f64>>,
    b: &Vec<Vec<f64>>,
    size: usize,
) -> Vec<Vec<f64>> {
    let mut result = vec![vec![0.0; size]; size];
    // what does par_iter_mut does?
    result.par_iter_mut().enumerate().for_each(|(i, row)| {
        // entire row is put into a thread
        for j in 0..size {
            row[j] = (0..size).map(|k| a[i][k] * b[j][k]).sum();
        }
    });
    result
}

use rayon::ThreadPoolBuilder;

fn parallel_matrix_multiplication_tb(
    a: &Vec<Vec<f64>>,
    b: &Vec<Vec<f64>>,
    size: usize,
) -> Vec<Vec<f64>> {
    // Create a custom thread pool with the desired number of threads (e.g., 8 threads)
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    let mut result = vec![vec![0.0; size]; size];

    // Use the custom thread pool for parallel execution
    pool.install(|| {
        // installing the pool with 8 threads
        // hypothesis is the threads will spread on all cores
        // Nope, it did not spread as expected...
        result.par_iter_mut().enumerate().for_each(|(i, row)| {
            // Perform matrix multiplication in parallel
            for j in 0..size {
                row[j] = (0..size).map(|k| a[i][k] * b[j][k]).sum();
            }
        });
    });

    result
}

fn main() {
    println!("Matrix Multiplication: with threading");
    let size = 1000;

    let a = generate_matrix(size);
    let b = generate_matrix(size);

    let start = Instant::now();

    let result = parallel_matrix_multiplication_tb(&a, &b, size);

    let duration = start.elapsed();

    println!("Matrix multiplication completed in {duration:.2?}");
    println!("Result matrix size: {} X {}", result.len(), result[0].len());
}
