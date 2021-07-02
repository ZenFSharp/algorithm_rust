struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i != 0 && nums[i] <= nums[i - 1] {
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
            return;
        }
        let s = i - 1;

        i = nums.len() - 1;
        while nums[s] >= nums[i] {
            i -= 1
        }
        let l = i;

        let t = nums[s];
        nums[s] = nums[l];
        nums[l] = t;

        nums[s+1..].reverse();
    }
}

fn main() {
    let mut nums = vec![1,2,3,13,8,5,];
    Solution::next_permutation(&mut nums);
    println!("{:?}", nums);
}
