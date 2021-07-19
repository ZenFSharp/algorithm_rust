struct Solution {}
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let len = nums.len();
        let mut total = 0i64;
        let mut l = 0;
        let mut result = 1;

        let mut r = 1;
        while r < len {
            total += (nums[r] as i64 - nums[r - 1] as i64) * (r as i64 - l as i64);
            while total > k as i64 {
                total -= nums[r] as i64 - nums[l] as i64;
                l += 1;
            }
            result = if result > r - l + 1 {
                result
            } else {
                r - l + 1
            };

            r += 1;
        }

        result as i32
    }
}

fn main() {
    println!("{}", Solution::max_frequency(vec![1,4,8,13], 5));
}
