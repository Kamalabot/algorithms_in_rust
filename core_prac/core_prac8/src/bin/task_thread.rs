#![allow(warnings)]

use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Task Threads");
    let gbl_task = Arc::new(Mutex::new(vec![1, 7, 9, 10]));
    let mut dhls = vec![];

    for idx in 0..4 {
        let lcl_task = Arc::clone(&gbl_task);
        let th = thread::spawn(move || {
            let mut taskv = lcl_task.lock();
            taskv[idx] += thread_rng().gen_range(5..50);
            let wait = thread_rng().gen_range(1..5);
            thread::sleep(Duration::from_secs(wait as u64));
            println!("Thread {idx} completed...in {wait} secs")
        });
        dhls.push(th);
    }
    for dh in dhls {
        dh.join().unwrap();
    }
    println!("Updated task vectors: {:?}", gbl_task.lock())
}
