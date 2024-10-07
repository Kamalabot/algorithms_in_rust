#![allow(unused_variables)]
#![allow(warnings)]

// recurrence relation to find max treasure
// dp[i][j] = treasure[i][j] + max(dp[i - 1][j],
// dp[i][j - 1])
use bt_pattern_02::print_vector_map;

fn main() {
    println!("2d treasure map");
    let tmap = vec![
        vec![0, 2, 2, 1],
        vec![3, 1, 1, 2],
        vec![4, 0, 2, 1],
        vec![2, 3, 2, 0],
    ];
    // for idx in 0..tmap.len() {
    //     for jdx in 0..tmap[idx].len() {
    //         println!("{}", tmap[idx][jdx])
    //     }
    // }
    // print_map(tmap);
    print_vector_map(tmap.clone());
    println!("");
    let max_val = max_treasure(tmap);
    println!("Max value is: {}", max_val);
}

fn max_treasure(treasuremap: Vec<Vec<i32>>) -> i32 {
    let rows = treasuremap.len();
    let cols = treasuremap[0].len();
    // create mutable 2d dp table of size rows x cols
    let mut dp = vec![vec![0; cols]; rows];
    // set the base case
    dp[0][0] = treasuremap[0][0];
    // fill the top row, using the left side value
    for jdx in 1..cols {
        dp[0][jdx] = dp[0][jdx - 1] + treasuremap[0][jdx];
    }
    // fill the first column using the value above it
    for idx in 1..rows {
        dp[idx][0] = dp[idx - 1][0] + treasuremap[idx][0];
    }
    // using clone for intermediate printing
    // circumvents the barrowing challenge
    println!("Partly filled dp table");
    print_vector_map(dp.clone());
    for idx in 1..rows {
        for jdx in 1..cols {
            dp[idx][jdx] =
                treasuremap[idx][jdx] + std::cmp::max(dp[idx - 1][jdx], dp[idx][jdx - 1]);
        }
    }
    println!("Completely filled dp table");
    print_vector_map(dp.clone());
    dp[rows - 1][cols - 1]
}
