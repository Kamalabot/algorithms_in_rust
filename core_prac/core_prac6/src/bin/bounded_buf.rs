use rand::thread_rng;
use rand::Rng;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn pdc(idx: i32, sender: Sender<i32>) {
    loop {
        let item = thread_rng().gen_range(1..100);
        sender.send(item).unwrap();
        println!("Pdc {idx} produced {item}");
        thread::sleep(Duration::from_secs(2))
    }
}
fn con(idx: i32, recvr: Arc<Mutex<Receiver<i32>>>) {
    loop {
        if let Ok(rcpt) = recvr.lock().unwrap().recv() {
            println!("Consumer {idx} recieved {rcpt}");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn main() {
    println!("Multi producer and consumer spawns");

    let (tx, rx) = mpsc::channel();
    let rcvr_cloner = Arc::new(Mutex::new(rx));

    for idx in 0..3 {
        let tidx = tx.clone();
        thread::spawn(move || pdc(idx, tidx));
    }
    for cdx in 0..3 {
        // sender has clone but reciever don't have
        let rcdx = Arc::clone(&rcvr_cloner);
        thread::spawn(move || con(cdx, rcdx));
    }
    thread::sleep(Duration::from_secs(10));
}
