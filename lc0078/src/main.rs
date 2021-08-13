struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        for i in 0..nums.len(){
            result = get_subset(nums[i], &mut result);
        }        
        result
    }
}
pub fn get_subset(num: i32, subset: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut _subset = subset.clone();
    for _s in _subset.iter_mut() {
        _s.push(num)
    }
    _subset.append(subset);
    _subset
}
fn main() {
    println!("{:?}", Solution::subsets(vec![1, 2, 3]));
}
