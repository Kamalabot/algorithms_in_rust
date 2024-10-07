fn main() {
    println!("Grid Traveller:");
    println!("{}", grid_traveller(3, 3));
}

fn grid_traveller(m: usize, n: usize) -> usize {
    // how many ways can you reach bottom right
    // initialize a 2D dp table
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // base case
    dp[1][1] = 1;

    // enumerate and fill in the DP table
    // with the recurrence relation
    for idx in 1..m + 1 {
        for jdx in 1..n + 1 {
            if jdx == 1 && idx == 1 {
                continue;
            }
            dp[idx][jdx] = (if idx > 1 { dp[idx - 1][jdx] } else { 0 })
                + (if jdx > 1 { dp[idx][jdx - 1] } else { 0 });
        }
    }
    dp[m][n]
}
