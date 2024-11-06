#![allow(warnings)]
#![allow(deadcode)]

use parking_lot::Mutex;
use rand::{thread_rng, Rng};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Multi producer consumer");
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut pd_hdls = vec![];
    let mut co_hdls = vec![];
    // spawning three senders
    for sdx in 0..3 {
        let sentx = tx.clone();
        let sth = thread::spawn(move || {
            producer(sdx, sentx);
        });
        pd_hdls.push(sth);
    }
    let rctx = Arc::new(Mutex::new(rx));
    for cdx in 0..2 {
        let clone_rx = Arc::clone(&rctx);
        let cth = thread::spawn(move || {
            consumer(cdx, clone_rx);
        });
        co_hdls.push(cth);
    }

    for p in pd_hdls {
        p.join().unwrap()
    }
    for c in co_hdls {
        c.join().unwrap()
    }
}

fn producer(pdx: i32, sdr: Sender<i32>) {
    loop {
        let pval = thread_rng().gen_range(10..100);
        let psdr = sdr.send(pval).unwrap();
        println!("Sending psdr from {pdx} producer: {pval}");
        let pdur = thread_rng().gen_range(0..10) as u64;
        thread::sleep(Duration::from_secs(pdur));
    }
}
fn consumer(cdx: i32, rcr: Arc<Mutex<Receiver<i32>>>) {
    loop {
        let crcr = rcr.lock().recv().unwrap();
        println!("recieving psdr on {cdx} consumer: {crcr}");
        thread::sleep(Duration::from_secs(2));
    }
}
