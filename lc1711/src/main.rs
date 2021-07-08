struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = 0;
        let max = deliciousness.iter().max().unwrap() * 2 + 1;
        let mut map = HashMap::new();
        println!("{:?}",deliciousness);
        for d in deliciousness.iter() {
            let mut sum = 1;
            while sum < max {
                if &sum < d {
                    sum <<= 1;
                    continue;
                }
                if let Some(c) = map.get(&(sum - d)) {
                    count = (count + c) % 1000000007;
                }
                sum <<= 1;
            }
            if let Some(c) = map.insert(d, 1) {
                map.insert(d, c + 1);
            }
        }
        count
    }
}

fn main() {
    let result = Solution::count_pairs(vec![32,32,32,32,32,]);
    println!("{}", result);
}
