struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut max = nums[0];
        let mut c_max = nums[0];
        let mut i = 0;
        while i < len - 1 {
            i += 1;
            c_max = if c_max < 0 { nums[i] } else { nums[i] + c_max };
            max = if c_max > max { c_max } else { max };
        }
        max
    }
}

fn main() {
    let result = Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);
    println!("{}", result);
}
