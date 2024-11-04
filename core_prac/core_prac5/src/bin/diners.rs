fn main() {
    println!("diners and forks");
    let fork = Arc::new(Mutex::new(()));
    let forks = vec![fork; 5];
    let diners = vec![
        Philosopher {
            name: "Diner1".to_string(),
        },
        Philosopher {
            name: "Diner2".to_string(),
        },
        Philosopher {
            name: "Diner3".to_string(),
        },
        Philosopher {
            name: "Diner4".to_string(),
        },
        Philosopher {
            name: "Diner5".to_string(),
        },
    ];

    let handles: Vec<_> = diners
        .into_iter()
        .enumerate()
        .map(|(idx, din)| {
            let left = forks[idx].clone();
            let right = forks[(idx + 1) % 5].clone();
            thread::spawn(move || loop {
                din.eat(&left, &right);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn eat(&self, left: &Mutex<()>, right: &Mutex<()>) {
        let _left = left.lock().unwrap();
        let _right = right.lock().unwrap();
        println!("{} is eating", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} is done eating", self.name)
    }
}
