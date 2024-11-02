#![allow(warnings)]
use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, stdin, Read};

fn main() {
    println!("Hello, world!");
    let val1 = 57;
    let val2 = 98;

    let divres = divide_result(val1, val2).expect("val2 must be > 0");
    println!("Divres: {divres}");
    let mut str1 = "mary has a little lamp".to_owned();
    let str2 = &str1[2..5];

    println!("Str 2: {str2}");
    // cannot change the str1 as it is barrowed above
    // drop(str2);
    str1 = "new york sessions0".to_owned();

    let str2 = &str1[2..5];
    println!("str2 after mut: {str2}");

    // println!("Give me a word, I giv a number");
    // str1.clear();
    // stdin().read_line(&mut str1).unwrap();
    // println!("String is: {str1}");
    // println!("Here you go: {}", str1.len());

    // working with propagated error
    // let read_data = propagate_error();
    // match read_data {
    //     Ok(data) => println!("{}", data),
    //     Err(e) => println!("Error: {e}"),
    // }

    {
        // creating a scope
        let monitor1 = Monitor {
            price: 25.7,
            model: "Dell".to_owned(),
        };
        println!("{}", monitor1);
    }

    let router = Router {
        range: 579,
        power: 7.9,
    };

    println!("Router {}", router);

    let mut rtr2 = router; // creates copy as it is implemented
    let rtr3 = &router; // this barrow
    rtr2.range = 7;

    println!("rtr2: {}", rtr2);
    println!("rtr3: {}", rtr3);
    // will get dropped automaticallb
    show_case_thread();
    rtr2.next();
    println!("rtr2: {}", rtr2);
}

use std::sync::{Arc, Mutex};
use std::thread;

fn show_case_thread() {
    let counter = Arc::new(Mutex::new(9));
    let mut handles = vec![];
    let tasks = vec![5, 7, -6, 3];

    for t in tasks {
        let thrd_tsk = Arc::clone(&counter);
        let hdl = thread::spawn(move || {
            let mut localnum = thrd_tsk.lock().unwrap();
            println!("Processing: {}", t);
            *localnum *= t;
        });
        handles.push(hdl);
    }
    for hdl in handles {
        hdl.join();
    }
    println!("Final Counter: {}", counter.lock().unwrap())
}

fn longest<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn divide_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Div by 0".to_owned())
    } else {
        Ok(a / b)
    }
}

fn propagate_error() -> io::Result<String> {
    let mut contents = String::new();
    let mut fobj = File::open("/home/uberdev/jupy.log")?;
    fobj.read_to_string(&mut contents)?;
    Ok(contents)
}

// use the structs show the move, copy, drop and clone

#[derive(Debug)]
struct Monitor {
    price: f64,
    model: String,
}

impl Drop for Monitor {
    fn drop(&mut self) {
        println!("Dropping Monitor: {}", self.model)
    }
}

impl Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model {} price is {}", self.model, self.price)
    }
}

#[derive(Debug, Copy, Clone)]
struct Router {
    range: i32,
    power: f64,
}
use std::fmt::{Display, Write};
impl Display for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(
            f,
            "range: {} and power: {}",
            self.range, self.power
        )?)
    }
}

use std::iter::Iterator;

impl Iterator for Router {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.power += 2.5;
        if self.power <= 688.0 {
            Some(self.power)
        } else {
            None
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    neigh: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            neigh: Vec::new(),
        }))
    }
}

fn bfs(start: Rc<RefCell<Node>>) {
    let mut queue = vec![start.clone()];
    let mut visited = vec![start.clone()];

    while let Some(n) = queue.pop() {
        println!("Visited node with value: {}", n.borrow().value);
        for nei in &n.borrow().neigh {
            if !visited.contains(nei) {
                visited.push(nei.clone());
                queue.push(nei.clone());
            }
        }
    }
}

fn thread_task_updt() {
    let mut task_queue = Arc::new(Mutex::new(vec![5, 7, 9, 8]));
    // idea is to update all four elements in single go
    let mut threads = vec![];

    for t in 0..4 {
        let mut clone_queue = Arc::clone(&task_queue);
        let hdl = thread::spawn(move || {
            let mut tasks = clone_queue.lock().unwrap();
            if let Some(mut tsk) = tasks.pop() {
                println!("Processed: {}", tsk);
                tsk += 57;
            }
        });
        threads.push(hdl);
    }
    for hdl in threads {
        hdl.join();
    }

    for upt in *task_queue.lock().unwrap() {
        println!("New value: {}", upt);
    }
}
