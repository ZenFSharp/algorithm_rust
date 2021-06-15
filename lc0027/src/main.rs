struct Solution{}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut current = 0;
        let mut next = 0;
        while next < nums.len() {
            if nums[next] != val {
                nums[current] = nums[next];
                current += 1
            }
            next += 1;
        }
        current as i32
    }
}

fn main() {
    let result = Solution::remove_element(&mut vec!(3,2,2,3), 3);
    println!("{}",result);
}
