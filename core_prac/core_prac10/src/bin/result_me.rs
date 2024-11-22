fn main() {
    println!("Showing result in Action...");
    let div = divme(10, 5).unwrap();
    println!("Correct Div: {div}");
    let erdv = divme(59, 0);
    match erdv {
        Ok(val) => println!("Got value: {val}"),
        Err(e) => println!("Got error: {e}"),
    };
    println!("Using match to avoid unwrap")
}

fn divme(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("There goes b...".to_owned());
    }
    let f = a / b;
    Ok(f)
}
