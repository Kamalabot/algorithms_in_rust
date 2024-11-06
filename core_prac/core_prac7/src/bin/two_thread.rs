#![allow(warnings)]
#![allow(deadcode)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Two threads");
    let mut ctr = 5; // this is variable to be changed
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let inc_cln = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        inc_cln.send(-5).unwrap();
    });

    let dec_cln = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        dec_cln.send(15).unwrap();
    });

    for _ in 0..2 {
        let rcpt = rx.recv().unwrap();
        ctr += rcpt;
        println!("Rcd {rcpt} and ctr is:{ctr}");
    }
    println!("The final output is :{ctr}")
}
