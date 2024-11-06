#![allow(warnings)]
#![allow(deadcode)]

use parking_lot::Mutex;
use rand::random;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Producer and consumer with buffer to fill");
    let buffer: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel();
    let p_buf = Arc::clone(&buffer);
    let pthr = thread::spawn(move || producer(p_buf, tx));
    let c_buf = Arc::clone(&buffer);
    let cthr = thread::spawn(move || consumer(c_buf, rx));
    pthr.join().unwrap();
    cthr.join().unwrap();
}

fn producer(pro_buf: Arc<Mutex<Vec<i32>>>, sdr: Sender<i32>) {
    loop {
        let mut in_pro = pro_buf.lock();
        let item = random::<i32>() % 100;
        println!("Getting {item} ready to be pushed");
        in_pro.push(item);
        sdr.send(item).unwrap();
        thread::sleep(Duration::from_secs(2));
        drop(in_pro);
    }
}
fn consumer(con_buf: Arc<Mutex<Vec<i32>>>, rcr: Receiver<i32>) {
    loop {
        let mut con_in = con_buf.lock();
        if let Some(val) = con_in.pop() {
            let _ = rcr.recv().unwrap();
            println!("Popped out {val}");
            thread::sleep(Duration::from_secs(1));
        };
        println!("Waiting for items")
    }
}
