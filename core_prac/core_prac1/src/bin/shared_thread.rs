use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Shared Thread Variables");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // clone the counter
        let loc_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..500 {
                let mut num = loc_counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // we are using the global counter here
    println!("Final counter: {}", *counter.lock().unwrap());
}
