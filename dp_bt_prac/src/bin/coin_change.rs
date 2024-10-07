use core::f32;

fn main() {
    println!("Coin Change...");
    let coins = vec![1, 2, 5, 10];
    let amount = 25;
    println!(
        "{} amount will need {} coins",
        amount,
        coin_change(coins, amount)
    );
    #[allow(warnings)]
    let change = vec![9.1, 2.5, 7.5, 10.2];
    if let Some(min_change) = change.iter().copied().reduce(f32::min) {
        println!("The minimum value is: {}", min_change);
    } else {
        println!("There are no values in list")
    }
}

// recurrence relation is
// dp[idx] = min(dp[idx], dp[idx - coin] + 1)

fn coin_change(coins: Vec<i32>, amount: usize) -> i32 {
    // setup the dp table
    // you don't need to use f32 as MAX of i32 can
    // be directly used, unlike in Python
    let mut dp = vec![i32::MAX; amount + 1];
    // setup base case
    dp[0] = 0;
    // enumerating the available coins
    for co in coins.iter() {
        // enumerate the range between co and amount
        for idx in *co as usize..amount + 1 {
            dp[idx] = std::cmp::min(dp[idx], dp[idx - *co as usize] + 1)
        }
    }
    if dp[amount] != i32::MAX {
        dp[amount]
    } else {
        -1
    }
}
