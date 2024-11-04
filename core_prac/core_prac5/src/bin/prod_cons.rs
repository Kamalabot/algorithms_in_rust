use rand::random;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Single producer & consumer");
    // arced, mutex buffer
    let buffer = Arc::new(Mutex::new(Vec::new()));
    // sender + reciever
    let (tx, rx) = mpsc::channel();
    // cloned buffer
    let pdc_buffer = Arc::clone(&buffer);
    // sender and pdc_buffer to pdc_worker
    let pdc_hdl = thread::spawn(move || pdc_worker(tx, pdc_buffer));
    // cloned buffer
    let con_buffer = Arc::clone(&buffer);
    // recvr and con_buffer to con_worker
    let con_hdl = thread::spawn(move || con_worker(rx, con_buffer));
    // getting the thread to start
    pdc_hdl.join().unwrap();
    con_hdl.join().unwrap();
}

fn pdc_worker(sender: Sender<i32>, buffer: Arc<Mutex<Vec<i32>>>) {
    // eternal loop
    loop {
        let item = random::<i32>() % 100;
        // generate item and get lock on buffer
        let mut buf = buffer.lock().unwrap();
        if buf.len() < 5 {
            // when buf is less than 5 element
            buf.push(item);
            sender.send(item).unwrap();
            // why drop buffer
            drop(buf);
            thread::sleep(Duration::from_millis(100))
        }
        // } else {
        //     println!("Waiting out of loop");
        //     thread::sleep(Duration::from_millis(1000));
        //     continue;
        // }
    }
}

fn con_worker(recver: Receiver<i32>, buffer: Arc<Mutex<Vec<i32>>>) {
    loop {
        let mut buf = buffer.lock().unwrap();
        if let Some(item) = buf.pop() {
            println!("Consumed {item}");
            let recd = recver.recv().unwrap();
            println!("recieved items {recd}");
            drop(buf);
            thread::sleep(Duration::from_millis(500));
        }
    }
}
