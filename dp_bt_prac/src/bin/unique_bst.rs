fn main() {
    println!("Unique Binary Search Tree");
    println!("{}", num_trees(5));
}

fn num_trees(n: usize) -> i32 {
    let mut dp = vec![0; n + 1];

    // base cases
    dp[0] = 1;
    dp[1] = 1;

    // catalan number
    for i in 2..=n {
        for j in 0..i {
            dp[i] += dp[j] + dp[i - j - 1]
        }
    }
    dp[n]
}
