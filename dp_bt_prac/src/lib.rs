pub fn print_vector_map(tmap: Vec<Vec<i32>>) {
    // takes in array of 4 i32 vectors
    // and prints them in a spaced grid
    for elm in tmap.iter() {
        for jdx in elm {
            print!("{} ", jdx)
        }
        println!()
    }
}
pub fn print_map(tmap: [Vec<i32>; 4]) {
    // takes in array of 4 i32 vectors
    // and prints them in a spaced grid
    for elm in tmap.iter() {
        for jdx in elm.iter() {
            print!("{} ", jdx)
        }
        println!()
    }
}

pub fn print_mxn_grid(m: usize, n: usize) {
    for _j in 0..n + 1 {
        for _i in 0..m + 1 {
            print!("__");
        }
        print!("_");
        println!();
        for _i in 0..m + 1 {
            print!("| ");
        }
        print!("|");
        println!();
    }
    for _i in 0..m + 2 {
        print!("__");
    }
    println!();
}
