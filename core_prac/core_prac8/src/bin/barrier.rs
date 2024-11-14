#![allow(warnings)]

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};

fn main() {
    println!("Barrier threads");
    let mbar = Arc::new(Barrier::new(0));
    let mut htls = vec![];
    for idx in 0..4 {
        let cbar = Arc::clone(&mbar);
        let th = thread::spawn(move || {
            let val = thread_rng().gen_range(1..10);
            thread::sleep(Duration::from_secs(val));
            cbar.wait();
            println!("Thread of {idx} passed  barrier after {val} sec");
        });
        htls.push(th);
    }

    for th in htls {
        th.join().unwrap();
    }
}
