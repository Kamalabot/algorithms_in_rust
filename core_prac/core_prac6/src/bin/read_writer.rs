#![allow(warnings)]
#![allow(dead_code)]
use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let dlock = Arc::new(RwLock::new(0));

    let mut readers = vec![];

    for idx in 0..3 {
        let llck = Arc::clone(&dlock);
        let rdr = thread::spawn(move || loop {
            let _data = llck.read();
            println!("Reader {} is reading {:?}", idx, _data);
            thread::sleep(Duration::from_secs(1));
        });
        readers.push(rdr);
    }

    let wlck = Arc::clone(&dlock);
    let wtr = thread::spawn(move || loop {
        let mut_data = wlck.write();
        println!("One writer is writing");
        thread::sleep(Duration::from_secs(5));
    });
    for rd in readers {
        rd.join().unwrap();
    }

    wtr.join().unwrap();
}
