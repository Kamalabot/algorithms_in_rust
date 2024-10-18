use std::thread;

fn main() {
    //create vector with 100 numbers
    let numbers: Vec<i32> = (1..=100).collect();
    // declare the num of threads
    let num_threads = 4;
    // find the chunk_size
    let chunk_size = (numbers.len() as f32 / num_threads as f32).ceil() as usize;
    // declare the handles vector
    let mut handles = vec![];
    // loop on num_threads to spawn threads
    for i in 0..num_threads {
        // declare start and end of chunks on vectors
        let start = i * chunk_size;
        // to check whether end of the numbers has been reached
        // as we are applying ceiling for the chunk_size
        let end = std::cmp::min(start + chunk_size, numbers.len());
        // slice part of the numbers vector to process
        let num_to_process = numbers[start..end].to_vec();
        // thread handle spawned
        let handle = thread::spawn(move || {
            // here the processes are being taken care
            for num in num_to_process {
                println!("Thread {i} is processing {num}");
                std::thread::sleep(std::time::Duration::from_secs(5))
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
