use std::collections::HashSet;

struct Solution{}
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut flags = HashSet::new();
        for n in &nums {
            if n>&0 {
                flags.insert(n);
            }
        }
        let mut i = 1;
        loop{
            if let None = flags.get(&i){
                return i;
            }
            i+=1;
        }
    }
}

fn main() {
    let result = Solution::first_missing_positive(vec![6,5,8,9,4,2,1,3]);
    println!("{}",result);
}
