struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let n = target - nums[i];
            if !map.contains_key(&n) {
                map.insert(nums[i], i as i32);
            } else {
                result.push(*map.get(&n).unwrap());
                result.push(i as i32);
                return result;
            }
        }
        result
    }
}

fn main() {
    let nums = vec![-2, 7, 11, 15];
    let target = 9;

    let indexs = Solution::two_sum(nums, target);
    println!("{:?}", indexs);
}
