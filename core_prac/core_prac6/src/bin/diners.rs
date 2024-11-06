#![allow(warnings)]
#![allow(dead_code)]

use std::sync::Arc;
use std::{sync::Mutex, thread, time::Duration};

#[derive(Debug)]
struct Philosopher {
    name: String,
}

impl Philosopher {
    fn eat(&self, left: &Mutex<()>, right: &Mutex<()>) {
        let _l = left.lock().unwrap();
        let _r = right.lock().unwrap();
        println!("{} is eating", self.name);
        thread::sleep(Duration::from_secs(2));
        println!("{} has completed eating", self.name);
    }
}

fn main() {
    println!("Diners");
    let fork = Arc::new(Mutex::new(()));
    let forks = vec![fork; 5];
    let diners = vec![
        Philosopher {
            name: "Din1".to_owned(),
        },
        Philosopher {
            name: "Din2".to_owned(),
        },
        Philosopher {
            name: "Din3".to_owned(),
        },
        Philosopher {
            name: "Din4".to_owned(),
        },
        Philosopher {
            name: "Din5".to_owned(),
        },
    ];

    let mut handles = vec![];

    for (idx, din) in diners.into_iter().enumerate() {
        let left = forks[idx].clone();
        let right = forks[(idx + 1) % 5].clone();
        let hdls = thread::spawn(move || loop {
            din.eat(&left, &right);
        });
        handles.push(hdls);
    }
    for hdl in handles {
        hdl.join().unwrap()
    }
}
