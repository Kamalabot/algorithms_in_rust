fn main() {
    println!("Factorial Exercise...");
    let y = facto(5);
    println!("The factorial of 5: {y}")
}

fn facto(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    n * facto(n - 1)
}
