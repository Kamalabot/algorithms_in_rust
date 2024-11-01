use std::fmt::Pointer;
use std::fs::File;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn lib_test() {
    println!("library world");
}

pub fn div_result(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Div by 0 error".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn read_file(file_name: String) -> io::Result<String> {
    let mut store = String::new();
    let mut fobj = File::open(file_name)?;
    fobj.read_to_string(&mut store)?;
    Ok(store)
}

#[derive(Copy, Debug, Clone)]
pub struct Potr {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Square {
    width: i32,
    length: i32,
}

pub fn move_copy_barrow() {
    let pt1 = Potr { x: 12, y: 27 };
    let mut pt2 = pt1; // will be copied
    pt2.x = 267;
    println!("{:?} and {:?}", pt1, pt2);
    let sq1 = Square {
        width: 12,
        length: 27,
    };
    let sq2 = sq1; // will be moved, as no copy trait
                   // println!("{:?}", sq1)
    let sq3 = &sq2;
    println!("{:?}", sq3)
}

pub fn concurrent_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let ctr = Arc::clone(&counter);
        let hdl = thread::spawn(move || {
            let mut num = ctr.lock().unwrap();
            *num += 1;
        });
        handles.push(hdl);
    }
    for hdl in handles {
        hdl.join().unwrap()
    }
    println!("Final result: {}", counter.lock().unwrap())
}

use std::rc::Rc;
// creating a single threaded Rc and RefCell
pub struct Node {
    value: i32,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

pub fn create_tree() -> Rc<Node> {
    Rc::new(Node {
        value: 10,
        left: Some(Rc::new(Node {
            value: 5,
            left: None,
            right: None,
        })),
        right: Some(Rc::new(Node {
            value: 55,
            left: None,
            right: None,
        })),
    })
}

pub enum Calculation {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn stringing() {
    let code_as_string = stringify!(let x = 10 + 20;);
    println!("{}", code_as_string);
}

pub struct MyResource;

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("MyResource is being dropped");
    }
}
