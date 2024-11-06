#![allow(warnings)]
#![allow(dead_code)]

use rand::random;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Producer and consumer filling and consuming");
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let vecbuf: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
    let pdc_buf = Arc::clone(&vecbuf);
    let pdc_hdl = thread::spawn(move || producer(pdc_buf, tx));
    let con_buf = Arc::clone(&vecbuf);
    let con_hdl = thread::spawn(move || consumer(con_buf, rx));
    pdc_hdl.join().unwrap();
    con_hdl.join().unwrap();
}

fn producer(buf: Arc<Mutex<Vec<i32>>>, pdc: Sender<i32>) {
    loop {
        let mut loc_buf = buf.lock().unwrap();
        let num = random::<i32>() % 100;
        loc_buf.push(num);
        println!("Sending {} num to push into ", num);
        pdc.send(num).unwrap();
        drop(loc_buf);
        thread::sleep(Duration::from_secs(1));
    }
}
fn consumer(buf: Arc<Mutex<Vec<i32>>>, con: Receiver<i32>) {
    loop {
        let mut con_buf = buf.lock().unwrap();
        if let Some(val) = con_buf.pop() {
            let crd = con.recv().unwrap();
            println!("Printing popep value: {}", val);
            thread::sleep(Duration::from_secs(2));
        }
    }
}
