fn main() {
    println!("Climbing Stairs...");
    println!("There are {} ways to climb {} stairs", climb_stairs(5), 5)
}
// it is fibbonacci in disguise
fn climb_stairs(n_stairs: usize) -> i32 {
    // return if single stair
    if n_stairs == 1 {
        return 1;
    }
    // establish the dp
    let mut dp = vec![0; n_stairs + 1];

    // establish base case
    dp[1] = 1;
    dp[2] = 2;
    // observe the enumeration starts at 3
    for idx in 3..n_stairs + 1 {
        dp[idx] = dp[idx - 1] + dp[idx - 2];
    }
    dp[n_stairs]
}
