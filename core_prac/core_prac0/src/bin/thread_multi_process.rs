use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn thread_task(thread_id: usize, tx: mpsc::Sender<String>) {
    thread::sleep(Duration::from_secs(1));
    let result = format!("Thread-{thread_id} finished");
    tx.send(result).unwrap();
}
fn main() {
    let (tx, rx) = mpsc::channel();
    // holds the external threads
    let mut handles = vec![];
    // two external threads are spawned
    for pid in 0..2 {
        let tx_clone = tx.clone();
        // each of the external thread spawns threads
        let ext_hdl = thread::spawn(move || {
            // whatever is done here is a seperate thread
            let mut thread_handles = vec![];
            // here the thread spawning happens internally
            for tdx in 0..3 {
                let tx_clone = tx_clone.clone();
                let thread_id = pid * 3 + tdx;
                let int_hdl = thread::spawn(move || {
                    thread_task(thread_id, tx_clone);
                });
                // internal threads are stored
                thread_handles.push(int_hdl);
            }

            // wait for all internal threads to complete
            for hdl in thread_handles {
                // internal threads are waiting for complete
                hdl.join().unwrap();
            }
        });
        // 2 external threads are pushed
        handles.push(ext_hdl);
    }

    for ext in handles {
        // 2 external threads are waited for completion
        ext.join().unwrap();
    }
    for rcv in rx {
        // print the recieved data
        println!("{}", rcv);
    }
    println!("All tasks done...")
}
