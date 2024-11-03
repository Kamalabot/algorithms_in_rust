fn main() {
    println!("Hello, world!");
    let fact1 = fact(6);
    println!("fact of 6: {fact1}");
    let mut cache = HashMap::new();
    let fact2 = facthm(6, &mut cache);
    println!("fact with cache of 6: {fact2}");
    // how will i create two different life times?
    {
        let x = 25;
        let b = 57;
        print_var(&x, &b)
    }
    // show_thread();
    // match show_err_prop() {
    //     Ok(data) => println!("{}", data),
    //     Err(e) => eprintln!("{}", e),
    // }
    // let ptr2: Ptr;
    {
        let ptr1 = Ptr {
            range: "tomorrow",
            price: 68.6,
        };
        println!("{}", ptr1);
        // ptr2 = ptr1;
    }
    // println!("{}", ptr2)
    let _nd1 = Node {
        val: 25,
        neigh: Rc::new(RefCell::new(Vec::new())),
    };
    // need to remember that types that
    // implicitly copy or clone will be
    // Fn closure only. When using String / Vec
    // it becomes FnMut / FnOnce
    // Fn closure
    {
        let c1 = 6;
        let k2 = 9;
        let fkl = || {
            println!("{} {}", c1, k2);
        };
        fkl();
        fkl();
    }
    // FnOnce
    {
        let m1 = "name1".to_owned();
        let mvkl = move || {
            let x = m1;
            println!("{}", x);
        };
        mvkl();
        // mvkl();
    }
    {
        let mut m1 = String::from("78");
        let mut mutkl = move || {
            let m2 = '5';
            m1.push(m2);
            println!("Id is {:?}", m1);
        };
        mutkl();
        mutkl();
    }
}
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    val: i32,
    neigh: Rc<RefCell<Vec<Node>>>,
}

// fn traverse_node(start: Node) {
//     let mut visited = vec![start.clone()];
//     let mut store = vec![start.clone()];

//     while let Some(node) = store.pop() {
//         // push node in to visited if not already present
//         if !visited.contains(&node) {
//             visited.push(node)
//         }
//         store.push(node.neigh)
//     }
// }

use std::fmt::Display;

#[derive(Debug)]
struct Ptr<'a> {
    range: &'a str,
    price: f32,
}

impl<'a> Display for Ptr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} and price {}", self.range, self.price)
    }
}

impl<'a> Drop for Ptr<'a> {
    fn drop(&mut self) {
        println!("{} named with price {} is dropping", self.range, self.price)
    }
}

use std::fs::File;
use std::io::{self, Read};

fn show_err_prop() -> io::Result<String> {
    let mut content = String::new();
    let mut fboj = File::open("/home/uberdev/history.txt")?;
    fboj.read_to_string(&mut content)?;
    Ok(content)
}

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn show_thread() {
    println!("Getting threads to work");
    let taskque = Arc::new(Mutex::new(vec![5, 7, 8, 9]));
    let mut threads = vec![];

    for idx in 0..4 {
        let locque = Arc::clone(&taskque);
        let hdl = thread::spawn(move || {
            let mut getloc = locque.lock().unwrap();
            // increment locque[i]
            getloc[idx] += 5;
            println!("Sleep on thread {idx}");
            sleep(Duration::from_secs(2))
        });
        threads.push(hdl);
    }

    for idl in threads {
        idl.join().unwrap()
    }

    println!("Final vector: {:?}", taskque.lock().unwrap());
}

fn print_var(x: &i32, y: &i32) {
    println!("{}{}", x, y)
}
use std::collections::HashMap;

fn fact(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn facthm(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&val) = cache.get(&n) {
        val
    } else if n == 1 {
        1
    } else {
        let temp = n * facthm(n - 1, cache);
        cache.insert(n, temp);
        temp
    }
}

// function with generics & trait bounds
use std::fmt::Debug;
use std::ops::Add;
fn traitme<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Debug + Copy + PartialOrd,
{
    a + b
}
