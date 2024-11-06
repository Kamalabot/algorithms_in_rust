#![allow(warnings)]
#![allow(dead_code)]

use rand::random;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Threads that hits barrier and passes");
    barrier_hit();
}

fn barrier_hit() {
    let bar = Arc::new(Barrier::new(0));

    let mut hdls = vec![];

    for idx in 0..4 {
        let barloop = Arc::clone(&bar);
        let hdl = thread::spawn(move || {
            let rnd_time = random::<u64>() % 100 / 10;
            println!("Thread {} put to wait for {} sec", idx, rnd_time);
            thread::sleep(Duration::from_secs(rnd_time));
            barloop.wait();
            println!("Thread {} passed barloop", idx);
        });
        hdls.push(hdl);
    }
    for dh in hdls {
        dh.join().unwrap();
    }
}
