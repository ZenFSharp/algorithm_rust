struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        while i + 1 < nums.len() && nums[i] < nums[i + 1] {
            i += 1;
        }
        let mut nums = nums;
        let len = nums.len();
        nums[..=i].reverse();
        nums[i + 1..].reverse();
        nums.reverse();

        match nums.binary_search(&target) {
            Err(_) => -1,
            Ok(n) => {
                if n + i + 1 < len {
                    (n + i + 1) as i32
                } else {
                    (n + i + 1 - len) as i32
                }
            }
        }
    }
}
fn main() {
    let result = Solution::search(vec![0, 1, 2, 4, 5, 6, 7, ], 0);
    println!("{}", result);
}
