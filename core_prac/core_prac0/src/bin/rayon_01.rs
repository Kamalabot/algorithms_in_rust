use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();
    let num_threads = 6;

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    pool.install(|| {
        numbers.par_iter().for_each(|num| {
            println!("Processing Num: {}", num);
            sleep(Duration::from_millis(1500));
        })
    })
    // numbers.par_iter().for_each(|num| {
    //     println!("Processing: {}", num);
    //     sleep(Duration::from_millis(1500));
    // })
}
