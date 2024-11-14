#![allow(warnings)]

use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread;
use std::time::Duration;

fn producer(sdr: Sender<i32>, bfr: Arc<Mutex<Vec<i32>>>) {
    println!("Entering Producer");
    loop {
        let mut l_bfr = bfr.lock();
        let rval = thread_rng().gen_range(15..57);
        // let wval: u64 = thread_rng().gen_range(5..10);
        l_bfr.push(rval);
        sdr.send(rval).unwrap();
        println!("getting buffer len: {:?}", l_bfr.len());
        thread::sleep(Duration::from_secs(1));
        drop(l_bfr);
    }
}

fn consumer(rcr: Receiver<i32>, bfr: Arc<Mutex<Vec<i32>>>) {
    println!("Entering Consumer");
    loop {
        let mut l_bfr = bfr.lock();
        if let Some(bufval) = l_bfr.pop() {
            // let wval: u64 = thread_rng().gen_range(5..10);
            let rc_val = rcr.recv().unwrap();
            let buf_len = l_bfr.len();
            println!("buffer len: {buf_len}");
            thread::sleep(Duration::from_secs(2));
        }
    }
}
fn main() {
    println!("Producer Consumers...");
    let mut buffer: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
    let (sx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let prod_buf = Arc::clone(&buffer);
    thread::spawn(move || producer(sx, prod_buf))
        .join()
        .unwrap();
    let con_buf = Arc::clone(&buffer);
    let cth = thread::spawn(move || consumer(rx, con_buf));
    cth.join().unwrap();
}
