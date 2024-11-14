#![allow(warnings)]

use parking_lot::RwLock;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Read write");
    let mut rw_main = Arc::new(RwLock::new(0));
    let mut rthls = vec![];
    for ind in 0..3 {
        let cl_rw = Arc::clone(&rw_main);
        let rth = thread::spawn(move || {
            let reader = cl_rw.read();
            println!("Reader read: {reader}");
        });
        rthls.push(rth);
    }
    let cl_w = Arc::clone(&rw_main);
    thread::spawn(move || {
        let rval = thread_rng().gen_range(10..566);
        println!("writing {rval}");
        *cl_w.write() = rval;
    })
    .join()
    .unwrap();
    for th in rthls {
        th.join().unwrap();
    }
    println!("Final rwlock val: {:?}", rw_main)
}
