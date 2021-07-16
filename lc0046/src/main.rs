struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len == 1 {
            return vec![vec![nums[0]]];
        }

        let mut result = vec![];
        let mut i = 0;
        while i < len {
            let mut _nums = nums.clone();
            _nums.remove(i);
            let ps = Solution::permute(_nums);
            for mut n in ps.into_iter() {
                n.push(nums[i]);
                result.push(n);
            }
            i += 1
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::permute(vec![1,2,3,4]));
}
