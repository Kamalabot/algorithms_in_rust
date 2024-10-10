fn main() {
    println!("Painting fence");
    let n = 3;
    let k = 2;
    println!("{}", paint_fence(n, k))
}

fn paint_fence(n: usize, k: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return k;
    }
    // creating dp array
    let mut dp: Vec<usize> = vec![0; n + 1];
    // base case
    dp[1] = k;
    // iterating the dp
    for idx in 2..=n {
        dp[idx] = (k - 1) * (dp[idx - 1] + dp[idx - 2])
    }
    println!("{:?}", dp.clone());
    dp[n]
}

// dp[idx] = (k - 1) * (dp[idx - 1] + dp[idx - 2])
