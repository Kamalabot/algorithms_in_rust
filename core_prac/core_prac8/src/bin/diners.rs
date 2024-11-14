#![allow(warnings)]

// use parking_lot::Mutex;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Diner {
    name: String,
}

impl Diner {
    fn new(text: String) -> Self {
        Diner { name: text }
    }

    fn eat(&self, left: &Mutex<()>, right: &Mutex<()>) {
        println!("The {} trying to eat", &self.name);
        let _l = left.lock();
        let _r = right.lock();
        println!("The {} is eating", &self.name);
        thread::sleep(Duration::from_secs(2));
        println!("The {} is completed eating", &self.name);
    }
}
fn main() {
    println!("Diners dining");
    let fork = Arc::new(Mutex::new(()));
    let forks = vec![fork; 5];
    let diners = vec![
        Diner::new("name1".to_owned()),
        Diner::new("name2".to_owned()),
        Diner::new("name3".to_owned()),
        Diner::new("name4".to_owned()),
        Diner::new("name5".to_owned()),
    ];
    let mut dthls = vec![];
    for idx in 0..5 {
        let lf = forks[idx].clone();
        let rf = forks[(idx + 1) % 5 as usize].clone();
        let dinerth = diners[idx].clone();
        let ths = thread::spawn(move || loop {
            dinerth.eat(&lf, &rf)
        });
        dthls.push(ths);
    }
    for th in dthls {
        th.join().unwrap();
    }
}
