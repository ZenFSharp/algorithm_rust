struct Solution {}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();
        match nums.binary_search(&target) {
            Ok(one) => {
                let mut c = one as i32;                
                while c >= 0 && nums[c as usize] == target {
                    c -= 1;
                }                
                result.push(c + 1);

                c = one as i32;
                while c < nums.len() as i32 && nums[c as usize] == target {
                    c += 1;
                }
                result.push(c - 1);
            }
            _ => {
                result.push(-1);
                result.push(-1);
            }
        }
        result
    }
}

fn main() {
    let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 7);
    println!("{:?}", result);
}
