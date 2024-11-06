#![allow(warnings)]
#![allow(deadcode)]

use rand::random;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hitting barrier");
    let main_bar = Arc::new(Barrier::new(0));
    let mut hdls = vec![];
    for idx in 0..3 {
        let th_bar = Arc::clone(&main_bar);
        let thr = thread::spawn(move || {
            let secs = random::<u64>() % 100 / 10;
            println!(" {idx} is Waiting for {secs} secs");
            thread::sleep(Duration::from_secs(secs));
            th_bar.wait();
            println!(" {idx} is through");
        });
        hdls.push(thr);
    }
    for h in hdls {
        h.join().unwrap();
    }
}
