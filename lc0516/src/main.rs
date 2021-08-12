struct Solution {}
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = (0..n)
            .map(|_| (0..n).map(|_| 0).collect())
            .collect::<Vec<Vec<i32>>>();
        let chars = s.chars().collect::<Vec<char>>();
        for i in 0..n {
            dp[n - i - 1][n - i - 1] = 1;
            let c1 = chars[n - i - 1];
            for j in n - i..n {
                let c2 = chars[j];
                dp[n - i - 1][j] = if c1 == c2 {
                    dp[n - i][j - 1] + 2
                } else {
                    std::cmp::max(dp[n - i][j], dp[n - i - 1][j - 1])
                }
            }
        }
        dp[0][n - 1]
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome_subseq("bbbab".to_owned())
    );
}
