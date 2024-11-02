#![allow(warnings)]
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

    println!("Give me a word, I giv a number");
    str1.clear();
    stdin().read_line(&mut str1).unwrap();
    println!("String is: {str1}");
    println!("Here you go: {}", str1.len());

    // working with propagated error
    let read_data = propagate_error();
    match read_data {
        Ok(data) => println!("{}", data),
        Err(e) => println!("Error: {e}"),
    }
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
    let mut fobj = File::open("~/jupy.log")?;
    fobj.read_to_string(&mut contents)?;
    Ok(contents)
}
