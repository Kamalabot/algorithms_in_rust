#![allow(dead_code)]
#![allow(warnings)]

use std::fs::File;
use std::io::{self, stdin, Read};

fn main() {
    // println!("Hello, world!");
    // let mut factcache = HashMap::new();
    // let fact6 = facto_cache(6, &mut factcache);
    // println!("Fact 6: {fact6}");

    // let n = 55;
    // let x = 11;
    // let res = div_by_zero(n, x).unwrap();

    // println!("Res is : {res}")
    // let mut getstr = String::new();
    // println!("give an input: ");
    // stdin().read_line(&mut getstr).unwrap();
    // let pathstr = "/home/uberdev/.bashrc".to_owned();
    // if let Ok(content) = read_prop_error(getstr) {
    //     println!("File data: {content}");
    // } else {
    //     println!("Error:")
    // }
    // {
    //     let mut tria1 = Tria {
    //         base: 62,
    //         top: "Everest",
    //     };

    //     println!("Lets make a tria: {tria1}");

    //     tria1.next();

    //     println!("After next tria is: {tria1}")
    // }

    // let makesummul = make_mul(56.8, 65.2, 32.1);
    // task_threads();
    talking_threads();
}

use std::collections::HashMap;

fn facto_cache(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if n == 1 {
        1
    } else if cache.contains_key(&n) {
        *cache.get(&1).unwrap()
    } else {
        let temp = n * facto_cache(n - 1, cache);
        cache.insert(n, temp);
        temp
    }
}

fn div_by_zero(numer: i32, deno: i32) -> Result<i32, String> {
    if deno == 0 {
        Err("Div by 0".to_owned())
    } else {
        Ok(numer / deno)
    }
}

fn read_prop_error(path: String) -> io::Result<String> {
    println!("Path is {path}");
    let trimmed = path.trim();
    let mut data = String::new();
    let mut fbj = File::open(trimmed)?;
    fbj.read_to_string(&mut data)?;
    Ok(data)
}

use std::fmt::Display;
use std::iter::Iterator;

#[derive(Debug)]
struct Tria<'a> {
    base: i32,
    top: &'a str,
}

impl<'a> Display for Tria<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Triangle base:{} with top at {}", self.base, self.top)
    }
}

impl<'a> Drop for Tria<'a> {
    fn drop(&mut self) {
        println!("There goes tria at {}", self.top)
    }
}

impl<'a> Iterator for Tria<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.base += 1;
        Some(self.base)
    }
}

use std::ops::{Add, Mul};

// fn make_mul<T: Add<Output = T> + Mul<Output = T> + Display>(idx: T, bdx: T, addval: T) {
fn make_mul<T>(idx: T, bdx: T, addval: T)
where
    T: Add<Output = T> + Mul<Output = T> + Display,
{
    let mul_res = idx * bdx;
    println!("Multiply value {}", mul_res);
    println!("Multiplied and added {}", mul_res + addval);
}

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Tasking Threads
/// # Purpose
///
/// Shows multiple threads incrementa
/// task vector value
///
/// - arguments: No Arguments
///
fn task_threads() {
    let gbl_tasks = Arc::new(Mutex::new(vec![5, 7, 9, 3]));
    let mut threads = vec![];

    for idx in 0..4 {
        let mut local_task = Arc::clone(&gbl_tasks);
        // loop is going to spawn four threads
        let hdl = thread::spawn(move || {
            // each element is individually processed
            let mut task = local_task.lock().unwrap()[idx];
            let mut task_array = local_task.lock().unwrap();
            task += 5;
            println!("Task is at {task}");
            task_array[idx] += 5;
        });
        // each thread is pushed into handles
        threads.push(hdl);
    }

    for th in threads {
        th.join().unwrap()
    }

    println!("Final tasks: {:?}", gbl_tasks.lock().unwrap())
}
/// Talking Threads
/// # Purpose
///
/// Shows how two threads increment & decrement a
/// single value
///
/// sender is cloned before sending into the threads
///
/// - arguments: No Arguments
///
fn talking_threads() {
    let mut counter = 5;
    let (rx, tx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let tinc = rx.clone();
    thread::spawn(move || {
        tinc.send(5).unwrap();
    });
    let tdec = rx.clone();
    thread::spawn(move || {
        tdec.send(-15).unwrap();
    });
    // tinc.join().unwrap();
    // tdec.join().unwrap();
    for _ in 0..2 {
        let val = tx.recv().unwrap();
        counter += val;
    }

    println!("Final counter: {counter}")
}
