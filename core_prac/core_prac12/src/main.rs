#![allow(unused)]

use std::fs::File;
use std::io::Write;
use std::io::{stdin, Read};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    println!("Hello, world!");
    // lets see if I can get the input working
    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // println!("{}", input);
    // write_file();
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    let fib = fibonacci(20);
    println!("{}", fib);
    make_file();

    let per = Person {
        name: "John".to_string(),
        age: 30,
    };

    let serialized = serde_json::to_string(&per).unwrap();
    println!("{}", serialized);

    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("{}", deserialized.name);

    let json = r#"
    {
        "name": "John",
        "age": 30
    }
    "#;
    println!("Used Json: {}", json);
    let deserialized: Person = serde_json::from_str(json).unwrap();
    println!("{}", deserialized.name);
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut idx = 0;
    loop {
        // println!("{}", a);
        let c = a + b;
        a = b;
        b = c;
        idx += 1;
        if idx > n {
            break;
        }
    }
    a
}

fn write_file() {
    let mut file = File::create("test.txt").unwrap();
    let mut locstr = String::new();
    stdin().read_line(&mut locstr).unwrap();
    file.write_all(locstr.as_bytes()).unwrap();
}
fn make_file() -> std::io::Result<()> {
    let mut f = File::options().append(true).open("test.txt")?;
    writeln!(&mut f, "new line")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
