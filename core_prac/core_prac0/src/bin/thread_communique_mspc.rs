use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let producer_thread = thread::spawn(move || {
        // let vals = vec![1, 2, 3, 4, 5];
        let vals: Vec<i32> = (0..100).collect();
        let mut plod = vec![];
        for val in vals {
            plod.push(val);
            if plod.len() > 3 {
                println!("Producer sending: {}", plod.len());
                tx.send(plod.clone()).unwrap();
                plod.clear()
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    let consumer_thread = thread::spawn(move || {
        for recieved in rx {
            println!("Consumer: recieved {recieved:?}");
        }
    });
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
