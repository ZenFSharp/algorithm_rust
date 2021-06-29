struct Solution {}
use std::convert::TryFrom;
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();

        let mut nums:Vec<u32> = vec![];
        let mut column_number = column_number as u32;
        while column_number > 26 {
            let n = column_number % 26;
            if n == 0 {
                nums.push(26);
                column_number = column_number / 26 - 1;
            } else {
                nums.push(n);
                column_number /= 26;
            }
        }
        if column_number > 0 {
            nums.push(column_number);
        }

        while let Some(n) = nums.pop(){
            // result.push(char::from_u32('A' as u32 +n-1).unwrap()); leetcode连1.0都不到?
            result.push(char::try_from('A' as u32 +n-1).ok().unwrap());
        }
        result
    }
}

fn main() {
    let result = Solution::convert_to_title(676);
    println!("{}", result);
}
