#![allow(warnings)]
#![allow(deadcode)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Task threads");
    let gbl_tasks = Arc::new(Mutex::new(vec![5, 6, 7, 9]));
    let mut hdls = vec![];

    for idx in 0..4 {
        let task_cln = Arc::clone(&gbl_tasks);
        let hdl = thread::spawn(move || {
            let mut tasks = task_cln.lock().unwrap();
            tasks[idx] += 26;
        });
        hdls.push(hdl);
    }
    for hdl in hdls {
        hdl.join().unwrap()
    }
    println!("Final tasks: {:?}", gbl_tasks.lock().unwrap());
}
