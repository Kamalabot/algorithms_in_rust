use rand::{thread_rng, Rng};
use tokio::task;

async fn print_num_wait(num: i32, sec: u64) {
    // println!("Waiting for random time to print: {num}");
    // planning to have a variable sleep for each process
    // random way is difficult to understand
    // let sleep_sec = thread_rng().gen_range(1..=5);
    std::thread::sleep(std::time::Duration::from_secs(sec));
    println!("Waiting for {sec} secs after printing {num}");
}

#[tokio::main]
async fn main() {
    println!("Working with Tokio Task");
    let numbers = [1, 2, 3, 5, 7, 18, 20];
    // let numbers = [1, 2];
    let sleep_sec: Vec<u64> = vec![25, 7, 35, 15, 45, 3, 50];

    let mut handles = vec![];

    for (idx, num) in numbers.iter().enumerate() {
        let handle = task::spawn(print_num_wait(*num, sleep_sec[idx]));
        handles.push(handle);
    }
    for hand in handles {
        // we call each spawned task
        hand.await.unwrap()
    }
}
