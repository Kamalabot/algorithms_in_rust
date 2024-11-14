#![allow(warnings)]
use rand::{thread_rng, Rng};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

fn main() {
    // todo!("Lets stay calm");
    let (sx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut pthls = vec![];
    let mut cthls = vec![];
    for pid in 0..3 {
        let csx = sx.clone();
        let pth = thread::spawn(move || prod(csx, pid));
        pthls.push(pth);
    }
    let arc_rx = Arc::new(Mutex::new(rx));
    for kid in 0..3 {
        let crx = Arc::clone(&arc_rx);
        let pth = thread::spawn(move || cons(crx, kid));
        cthls.push(pth);
    }

    for th in pthls {
        th.join().unwrap();
    }
    for th in cthls {
        th.join().unwrap();
    }
}

fn prod(sdr: Sender<i32>, pid: i32) {
    loop {
        let sv = thread_rng().gen_range(0..5);
        println!("Sending {sv} from {pid} id");
        sdr.send(sv).unwrap();
        thread::sleep(Duration::from_secs(2))
    }
}

fn cons(rcr: Arc<Mutex<Receiver<i32>>>, cid: i32) {
    loop {
        if let Ok(rcpt) = rcr.lock().unwrap().recv() {
            let sv = thread_rng().gen_range(0..5);
            println!("receiving {rcpt} from {cid} consumer thread");
            thread::sleep(Duration::from_secs(1))
        }
    }
}
