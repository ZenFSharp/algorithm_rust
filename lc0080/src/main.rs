struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut position = 1usize;
        let mut duplicate = false;

        let mut i = 1;
        while i < nums.len() {
            if nums[i] == nums[i - 1] {
                if duplicate {
                    i += 1;
                    continue;
                } else {
                    duplicate = true
                }
            } else {
                duplicate = false;
            }
            if position != i {
                nums[position] = nums[i];                
            }
            position += 1;
            i += 1;
        }

        position as i32
    }
}

fn main() {
    let mut v = vec![1,1,1,2,2,3];
    println!("{}", Solution::remove_duplicates(&mut v));
}
