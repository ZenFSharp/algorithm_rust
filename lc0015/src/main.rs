struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }
        let mut nums = nums;
        nums.sort();

        let mut i = 0;
        while i < nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            let target = 0 - nums[i];
            while j < k {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    j += 1;
                    continue;
                }
                while nums[j] + nums[k] > target && j < k - 1 {
                    k -= 1;
                    continue;
                }
                if nums[j] + nums[k] == target {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                }
                j += 1;
            }
            i += 1;
        }

        result
    }
}

fn main() {
    let result = Solution::three_sum(vec![]);
    println!("{:?}", result);
}
