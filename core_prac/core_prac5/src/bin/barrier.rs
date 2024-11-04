use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let bar = Arc::new(Barrier::new(3));

    let mut hdls = vec![];

    for idx in 0..3 {
        let barloop = Arc::clone(&bar);
        let bts = thread::spawn(move || {
            thread::sleep(Duration::from_secs(idx * 2));
            println!("Thread {idx} is at bar");
            barloop.wait();
            thread::sleep(Duration::from_secs(idx * 5));
            println!("Thread {idx} has passed bar");
        });
        hdls.push(bts);
    }

    for i in hdls {
        i.join().unwrap();
    }
}
