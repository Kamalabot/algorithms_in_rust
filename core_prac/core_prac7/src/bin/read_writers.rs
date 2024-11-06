#![allow(warnings)]
#![allow(deadcode)]

use parking_lot::RwLock;
use rand::random;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Readers and writers block...");
    let mut hdls = vec![];
    let rw = Arc::new(RwLock::new(2));
    // creatin reader
    for idx in 0..3 {
        let reader_rw = Arc::clone(&rw);
        let rth = thread::spawn(move || {
            let reader_val = reader_rw.read();

            thread::sleep(Duration::from_secs(2));
            println!("reading with lock in writer: {:?}", reader_val);
        });
        hdls.push(rth);
    }
    let writer_rw = Arc::clone(&rw);
    let wth = thread::spawn(move || {
        let mut writer = writer_rw.write();
        *writer = 57;
        println!("Writing with lock in writer");
        thread::sleep(Duration::from_secs(1));
    });

    wth.join().unwrap();
    for d in hdls {
        d.join().unwrap();
    }

    println!("Final rw lock value: {:?}", rw)
}
