use libc::{cpu_set_t, sched_setaffinity, CPU_SET};
use std::sync::Arc;
use std::thread;

fn set_cpu_affinity(thread_id: usize) {
    let mut cpuset = unsafe { std::mem::zeroed::<cpu_set_t>() }; // Initialize the CPU set with zeros

    // Set the CPU affinity to the core corresponding to the thread ID
    unsafe {
        CPU_SET(thread_id % num_cpus::get(), &mut cpuset); // Assign to a specific CPU core
        println!("Setting the cpu: {}", thread_id % num_cpus::get());
        let result = sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpuset);
        if result != 0 {
            eprintln!("Failed to set CPU affinity for thread {}", thread_id);
        }
    }
}

fn process_numbers(start: usize, end: usize, numbers: Arc<Vec<i32>>, thread_id: usize) {
    set_cpu_affinity(thread_id);
    for i in start..end {
        println!("Thread {thread_id} is processing number: {}", numbers[i]);
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn main() {
    // create a Atomic Ref Counted Vector storing 100 nums
    let numbers: Arc<Vec<i32>> = Arc::new((1..=100).collect());
    // decide the num of processes
    let num_process = 4;
    // get the chunk_size;
    let chunk_size = (numbers.len() as f32 / num_process as f32).ceil() as usize;
    let mut handles = vec![];
    for i in 0..num_process {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, numbers.len());
        let thread_numbers = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            process_numbers(start, end, thread_numbers, i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
