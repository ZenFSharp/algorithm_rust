struct Solution {}
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut furthest = 0;

        let mut i = 0;
        while i < len {
            if i <= furthest {
                furthest = std::cmp::max(furthest, i + nums[i] as usize);
                if furthest >= len - 1 {
                    return true;
                }
            }
            i += 1
        }
        false
    }
}
fn main() {
    println!("{}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
}
