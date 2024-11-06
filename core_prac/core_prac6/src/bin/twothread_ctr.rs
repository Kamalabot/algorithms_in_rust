#![allow(warnings)]
#![allow(dead_code)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    println!("Changing one counter with thread reciever");
    send_recv();
}

fn send_recv() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut ctr = 68;

    // there are two senders
    let tx_inc = tx.clone();
    thread::spawn(move || tx_inc.send(5).unwrap());
    let tx_dec = tx.clone();
    thread::spawn(move || tx_dec.send(-55).unwrap());
    for _ in 0..2 {
        let rcv = rx.recv().unwrap();
        ctr += rcv;
    }
    println!("Final counter is {ctr}")
}
