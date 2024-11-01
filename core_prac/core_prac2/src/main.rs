use core_prac2::{div_result, lib_test, move_copy_barrow, read_file, Calculation, MyResource};

fn main() {
    println!("Hello, world!");
    // lib_test();

    let maker: i32 = 25;

    let barrow_maker = &maker;

    println!("Maker is {maker}");
    println!("Barrowed Maker is {barrow_maker}");

    let mut copy_maker = maker;
    // Because i32 implements the Copy trait, it creates a
    // duplicate value rather than moving ownership. Changes to copy_maker do not affect maker.

    copy_maker += 5;
    println!("Copy Maker: {copy_maker}");

    println!("Maker now: {maker}");
    // learing how move, barrow and copy is working
    //
    dbg!(); // prints line num and file name

    dbg!(std::any::type_name::<i32>());

    let div_val = div_result(1.5, 0.5).expect("got error");
    println!("Div val is: {div_val}");
    // let div_val1 = div_result(1.5, 0.0).expect("got error");
    // expect will terminate
    let div_val1 = div_result(1.5, 0.0).unwrap_or(5.0);
    println!("Div val 1 is: {div_val1}");

    let mut stringme = "copystring";

    let mut copystring = stringme;

    stringme = "newcpy";
    copystring = "update stringme";
    // workin on &str wont have any impact, as the
    // string slices are immutable by default
    println!("Print copystring: {copystring}");
    println!("Print stringme: {stringme}");
    let fileme = "testfile".to_owned();

    match read_file(fileme) {
        Ok(contents) => println!("File contents: {contents}"),
        Err(e) => eprintln!("Probably no file: {e}"),
    }
    move_copy_barrow();
    use std::cell::RefCell;
    use std::rc::Rc;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    *value.borrow_mut() += 1; // Modify the value through one reference
    println!("Updated value: {}", *a.borrow()); // Access the updated value through another reference
    *value.borrow_mut() += 1; // Modify the value through one reference
    println!("Updated value: {}", *b.borrow()); // Access the updated value through another reference
    println!("Orig value: {:?}", *value); // Access the updated value through another reference
    let a = 56;
    let b = 6;
    let op = Calculation::Add;
    let out = match op {
        Calculation::Add => a + b,
        _ => a * b,
    };
    println!("Enum output: {out}");
    // println!("{}", stringify!(b));
    macro_rules! debug_print {
        ($val: expr) => {
            println!("{} = {:?}", stringify!($val), ($val));
        };
    }
    debug_print!(out);
    let _res = MyResource;
    println!("res created");
    println!("Yet to drop...");
    drop(_res);
}

// String Slices vs. Owned Strings: In Rust, a string literal
// (like "copystring") is a string slice (&str).
// If you want to modify it or own it, you need to
// use String, which is an owned heap-allocated string.

// Mutability: You can make a variable mutable,
// but it doesn't affect the immutability of string slices.
// To modify the contents, you need to work with a String.
