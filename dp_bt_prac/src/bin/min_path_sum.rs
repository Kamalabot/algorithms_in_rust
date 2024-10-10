fn main() {
    println!("Minimum path sum");
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!("{}", min_path_sum(grid)); // Output: 7
}
// Given a m x n grid filled with non-negative integers,
// the goal is to find a path from the top-left corner
// of the grid to the bottom-right corner such that the
// sum of the numbers along the path is minimized.
fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0; n]; m];
    // base case
    dp[0][0] = grid[0][0];
    // fill the first row
    for idx in 1..n {
        dp[0][idx] = dp[0][idx - 1] + grid[0][idx];
    }
    for jdx in 1..m {
        dp[jdx][0] = dp[jdx - 1][0] + grid[jdx][0];
    }
    for idx in 1..m {
        for jdx in 1..n {
            dp[idx][jdx] = std::cmp::min(dp[idx - 1][jdx], dp[idx][jdx - 1]) + grid[idx][jdx]
        }
    }
    dp[m - 1][n - 1]
}
