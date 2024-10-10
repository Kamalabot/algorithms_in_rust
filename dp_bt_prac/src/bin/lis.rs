fn main() {
    println!("Longest Increasing Subsequence");
    let lis = longest_inc_seq(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    println!("Longest increasing seq: {lis}")
}

fn longest_inc_seq(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // directly create the dp
    let mut dp = vec![1; n];
    // loop to create subproblems
    for idx in 1..n {
        // inner loop to check all elems
        // till idx
        for jdx in 0..idx {
            if nums[idx] > nums[jdx] {
                dp[idx] = std::cmp::max(dp[idx], dp[jdx] + 1);
            }
        }
    }
    dp.into_iter().max().unwrap()
}
// dp[idx] = max(dp[idx], dp[jdx] + 1)
