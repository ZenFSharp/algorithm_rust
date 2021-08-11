struct Solution {}
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let rows = grid.len();
        let columns = grid[0].len();
        let mut dp = (0..rows)
            .map(|_| (0..columns).map(|_| 0).collect())
            .collect::<Vec<Vec<i32>>>();
        dp[0][0] = grid[0][0];
        for i in 1..rows {
            dp[i][0] = dp[i - 1][0] + grid[i][0]
        }
        for j in 1..columns {
            dp[0][j] = dp[0][j - 1] + grid[0][j]
        }
        for i in 1..rows {
            for j in 1..columns {
                dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j]
            }
        }

        dp[rows - 1][columns - 1]
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1]])
    );
}
