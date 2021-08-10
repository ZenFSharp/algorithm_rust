struct Solution {}
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = nums.len();
        if len < 3 {
            return 0;
        }

        let mut x = nums[1] - nums[0];
        let mut l = 2;
        let mut i = 2;
        while i < len {
            let _x = nums[i] - nums[i - 1];
            if _x == x {
                l += 1;
            } else {
                if l != 2 {
                    result += (l - 2) * (l - 1) / 2
                }
                l = 2;
                x = _x;
            }
            i += 1
        }
        if l != 2 {
            result += (l - 2) * (l - 1) / 2
        }

        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 8, 9, 10, 11])
    );
}
