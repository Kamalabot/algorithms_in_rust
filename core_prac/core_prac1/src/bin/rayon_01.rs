use rayon::iter::plumbing::bridge_unindexed;
use rayon::iter::plumbing::UnindexedProducer;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut numbers: Vec<i32> = (1..=100).collect();
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
    });
    // numbers.par_iter().for_each(|num| {
    //     println!("Processing: {}", num);
    //     sleep(Duration::from_millis(1500));
    // })
    let par_sum = parallel_sum(&numbers);
    println!("Parallel sum: {par_sum}");
    let par_filter = par_map_filter(&numbers);
    println!("Parallel filter: {par_filter:?}");
    let par_sort = &numbers.par_sort();
    println!("Parallel Sorting: {par_sort:?}");
    let res: Vec<(&i32, &i32)> = PairChunks::new(&numbers).collect();
    for (a, b) in res {
        println!("Pair: ({}, {})", a, b);
    }
}

use rayon::iter::plumbing::*;

// Struct to hold a slice of type T and its lifetime.
struct PairChunks<'a, T: 'a> {
    slice: &'a [T], // Reference to a slice of T.
}

// Implementation of PairChunks to create a new instance.
impl<'a, T> PairChunks<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        PairChunks { slice } // Initialize PairChunks with the provided slice.
    }
}

// Implementation of ParallelIterator for PairChunks to enable parallel iteration.
impl<'a, T: Sync + 'a> ParallelIterator for PairChunks<'a, T> {
    type Item = (&'a T, &'a T); // Each item is a tuple of references to T.

    // Method to drive the iteration using a consumer.
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(self, consumer) // Bridge the consumer for unindexed processing.
    }
}

// Implementation of UnindexedProducer for PairChunks to allow splitting and folding.
impl<'a, T: Sync + 'a> UnindexedProducer for PairChunks<'a, T> {
    type Item = (&'a T, &'a T); // Same item type as in ParallelIterator.

    // Method to split the slice into two parts for parallel processing.
    fn split(self) -> (Self, Option<Self>) {
        if self.slice.len() < 2 {
            (self, None) // Return self if there are not enough elements to split.
        } else {
            let mid = self.slice.len() / 2; // Calculate the middle index.
                                            // Return two new PairChunks, one for each half of the slice.
            (
                PairChunks::new(&self.slice[..mid]),
                Some(PairChunks::new(&self.slice[mid..])),
            )
        }
    }

    // Method to fold the elements of the slice using a folder.
    fn fold_with<F>(self, mut folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        // Iterate over chunks of size 2 in the slice.
        for chunk in self.slice.chunks(2) {
            if let [a, b] = chunk {
                // Ensure chunk has exactly two elements.
                folder = folder.consume((a, b)); // Consume the pair (a, b) with the folder.
            }
        }
        folder // Return the updated folder.
    }
}

fn parallel_sum(arr: &[i32]) -> i32 {
    arr.par_iter().sum()
}

fn par_map_filter(arr: &[i32]) -> Vec<i32> {
    arr.par_iter()
        .map(|&x| x * 2)
        .filter(|&x| x % 2 == 0)
        .collect()
}
