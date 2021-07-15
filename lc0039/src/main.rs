struct Solution{}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination: Vec<i32> = Vec::new();
        let len = candidates.len();
        dfs(&mut result, &candidates, &mut combination,len, 0, target);
        result.reverse();
        result
    }
}

fn dfs(res: &mut Vec<Vec<i32>>, nums: &[i32], combination: &mut Vec<i32>, len: usize, index: usize, target: i32) { 
    if index >= len { 
        return; 
    }
    if target == 0 {
        res.push(combination.to_vec());
        return;
    }
    dfs(res, nums, combination, len, index + 1, target);
    if target - nums[index] >= 0 {
        combination.push(nums[index]);
        dfs(res, nums, combination, len, index,  target - nums[index]);    
        combination.pop();
    }
}

fn main() {
    let result = Solution::combination_sum(vec![2,3,6,7], 7);
    println!("{:#?}",result);
}
