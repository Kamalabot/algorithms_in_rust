use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn producer(id: usize, tx: mpsc::Sender<i32>) {
    loop {
        let item = rand::thread_rng().gen_range(1..100); // Generate a random number between 1 and 100
        tx.send(item).unwrap();
        println!("Producer {} produced {}", id, item);
        thread::sleep(Duration::from_secs(1));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        if let Ok(item) = rx.lock().unwrap().recv() {
            println!("Consumer {} consumed {}", id, item);
            thread::sleep(Duration::from_secs(2));
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn producers
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    // Spawn consumers
    for i in 0..2 {
        let rx = Arc::clone(&rx);
        thread::spawn(move || consumer(i, rx));
    }

    // Keep the main thread alive to let producer and consumer threads run
    thread::sleep(Duration::from_secs(15));
}
