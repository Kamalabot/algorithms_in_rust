fn main() {
    println!("knapsack solution");
    let weights = vec![1, 2, 3, 4];
    let values = vec![10, 15, 40, 25];
    let W = 8;
    let out = knapsack(weights, values, W);
    println!("Value out: {out}")
}
#[allow(warnings)]
fn knapsack(weights: Vec<i32>, values: Vec<i32>, W: usize) -> usize {
    let n = weights.len();
    // create the dp
    let mut dp = vec![vec![0; W + 1]; n + 1];
    // go through each value for smaller weig
    for idx in 1..=n {
        for wdx in 0..=W {
            if weights[idx - 1] as usize <= wdx {
                dp[idx][wdx] = std::cmp::max(
                    dp[idx - 1][wdx],
                    dp[idx - 1][wdx - weights[idx - 1] as usize] + values[idx - 1] as usize,
                );
            } else {
                dp[idx][wdx] = dp[idx - 1][wdx];
            }
        }
    }
    dp[n][W]
}
