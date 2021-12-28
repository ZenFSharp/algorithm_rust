struct Solution {}
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut _nums = nums.clone();
        _nums.sort();
        let mut cache = _nums[0];
        let mut _count = 0;
        let mut result = vec![vec![]];

        for num in _nums.into_iter() {
            let start = if cache == num { _count } else { 0 };
            let mut _subset = result[start..].to_vec();
            for sub in _subset.iter_mut(){
                sub.push(num);
            }
            _count = result.len();
            cache = num;
            result.append(&mut _subset);
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2]));
}
