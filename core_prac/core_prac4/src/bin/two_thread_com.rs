use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let mut counter = 0;

    let tx_inc = tx.clone();
    thread::spawn(move || {
        let inc = 5;
        for _ in 0..inc {
            tx_inc.send(5).expect("Failed to send increment");
        }
    });

    let tx_dec = tx.clone();
    thread::spawn(move || {
        let dec = 5;
        for _ in 0..dec {
            tx_dec.send(-1).expect("Failed sending decrement");
        }
    });
    for _ in 0..10 {
        counter += rx.recv().expect("Failed to recv");
    }

    println!("Final counter value: {}", counter)
}
