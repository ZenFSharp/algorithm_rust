struct Solution {}
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        costs
            .iter()
            .take_while(|&x| {
                coins -= x;
                coins >= 0
            })
            .count() as i32
    }
}

fn main() {
    let result = Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 0);
    println!("{}", result);
}
