fn main() {
    println!("Unique path with obstacles");
    let grid = vec![vec![0, 0, 2], vec![0, 1, 0], vec![0, 0, 0]];
    println!("{}", unique_path_obstacles(grid));
}

fn unique_path_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid.is_empty() || obstacle_grid[0].is_empty() {
        return 0;
    }
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    // start with dp vector
    let mut dp = vec![vec![0; n]; m];

    dp[0][0] = 1; // starting position

    for j in 1..n {
        if obstacle_grid[0][j] == 0 {
            dp[0][j] = dp[0][j - 1]
        } else {
            dp[0][j] = 0
        }
    }
    for i in 1..m {
        if obstacle_grid[i][0] == 0 {
            dp[i][0] = dp[i - 1][0]
        } else {
            dp[i][0] = 0
        }
    }
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            } else {
                dp[i][j] = 0;
            }
        }
    }
    dp[m - 1][n - 1]
}
