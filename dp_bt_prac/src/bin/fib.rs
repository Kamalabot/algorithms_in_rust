// Recurrence relation
// dp[i] = dp[i - 1] + dp[i - 2]

fn main() {
    println!("{}", fib(5));
    println!("{}", fib(10));
}

fn fib(n: usize) -> i32 {
    // create the table for dp
    let mut dp: Vec<i32> = vec![0; n + 1];
    // update the base cases
    dp[0] = 0;
    dp[1] = 1;
    for idx in 2..n + 1 {
        dp[idx] = dp.get(idx - 1).unwrap() + dp.get(idx - 2).unwrap();
    }
    dp[n]
}
