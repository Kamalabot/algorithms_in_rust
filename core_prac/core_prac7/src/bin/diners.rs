// #![allow(warnings)]
#![allow(dead_code)]

use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Diner {
    name: String,
}

impl Diner {
    fn eat(&self, left: &Mutex<()>, right: &Mutex<()>) {
        let _l = left;
        let _r = right;
        println!("Diner {} eatin", self.name);
        thread::sleep(Duration::from_secs(2));
        println!("Diner {} done", self.name);
    }
    fn new(given_name: String) -> Self {
        Diner { name: given_name }
    }
}

fn main() {
    println!("eating diners");
    let fork = Arc::new(Mutex::new(()));
    let forks = vec![fork; 5];
    let diners = [
        Diner::new("din1".to_owned()),
        Diner::new("din2".to_owned()),
        Diner::new("din3".to_owned()),
        Diner::new("din4".to_owned()),
        Diner::new("din5".to_owned()),
    ];
    let mut hdls = vec![];
    for d in 0..4 {
        let diner = diners[d].clone();
        let lf = forks[d].clone();
        let rf = forks[(d + 1) % 5].clone();
        let dth = thread::spawn(move || loop {
            diner.eat(&lf, &rf);
        });
        hdls.push(dth);
    }
    for th in hdls {
        th.join().unwrap();
    }
}
