fn main() {
    println!("Max coins from bursting baloons");
    let nums = vec![3, 1, 5, 8];
    let count = max_coins(nums);
    println!("count of max coin: {count}")
}

fn max_coins(nums: Vec<i32>) -> i32 {
    let mut loc_nums = nums.clone();
    loc_nums.push(1);
    loc_nums.insert(0, 1);
    println!("{:?}", loc_nums);
    println!("updated length: {}", loc_nums.len());
    let n = loc_nums.len();
    let mut dp = vec![vec![0; n]; n];

    for length in 2..n {
        for left in 0..(n - length) {
            let right: usize = left + length;
            for kdx in (left + 1)..right {
                println!(
                    "max right side:{}",
                    loc_nums[left] * loc_nums[kdx] * loc_nums[right]
                        + dp[left][kdx]
                        + dp[kdx][right]
                );
                println!("max left side:{}", dp[left][right]);
                dp[left][right] = dp[left][right].max(
                    loc_nums[left] * loc_nums[kdx] * loc_nums[right]
                        + dp[left][kdx]
                        + dp[kdx][right],
                )
            }
        }
    }
    for d in dp.clone() {
        for p in d {
            print!("{p}, ");
        }
        println!();
    }
    dp[0][n - 1]
}
