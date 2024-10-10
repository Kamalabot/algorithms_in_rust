fn main() {
    println!("Unbounded Knapsack");
    let weights = vec![1, 2, 3, 4];
    let values = vec![10, 15, 30, 25];
    println!("{}", unbounded_ks(weights, values, 2))
}
// dp[wt] = max(dp[wt], dp[wt - weights[idx]] + values[idx])

fn unbounded_ks(weights: Vec<i32>, values: Vec<i32>, target: usize) -> i32 {
    let n = weights.len();

    let mut dp = vec![0; target + 1];

    for wt in 1..=target {
        for idx in 0..n {
            if weights[idx] <= wt as i32 {
                dp[wt] = std::cmp::max(dp[wt], dp[wt - weights[idx] as usize] + values[idx])
            }
        }
    }
    dp[target]
}
