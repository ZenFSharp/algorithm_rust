struct Solution {}
impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let len = nums.len();
        let mut max = nums[0] + nums[len - 1];
        let mut i = 0;
        while i < len / 2 {
            max = if max > nums[i] + nums[len - 1 - i] {
                max
            } else {
                nums[i] + nums[len - 1 - i]
            };

            i += 1;
        }
        max
    }
}
fn main() {
    println!("{}", Solution::min_pair_sum(vec![1, 2, 3, 4]));
}
