fn main() {
    println!("Showcasing life times");
    let ft = 57;
    {
        let t = 9;
        show_lt(&ft, &t);
    }
}

fn show_lt<'t>(a: &'t i32, b: &'t i32) {
    let ic = b;
    {
        let db = ic + a;
        println!("Db is {db}")
    }
    let dc = a + b;
    println!("The dc: {dc}")
}
