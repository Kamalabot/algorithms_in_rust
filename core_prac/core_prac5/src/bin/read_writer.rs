use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(RwLock::new(0));

    let readers: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || loop {
                let _data = data.read().unwrap();
                println!("Reader {i} is reading");
                thread::sleep(Duration::from_secs(1))
            })
        })
        .collect();

    let data = Arc::clone(&data);
    let writer = thread::spawn(move || loop {
        let mut _data = data.write().unwrap();
        println!("Writer is writing");
        thread::sleep(Duration::from_secs(2));
    });

    for r in readers {
        r.join().unwrap();
    }
    writer.join().unwrap();
}
