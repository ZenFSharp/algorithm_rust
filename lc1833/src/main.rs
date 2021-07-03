struct Solution {}
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort();
        let mut count = 0;
        let mut sum = 0;
        while count < costs.len() && sum + costs[count] <= coins {
            sum += costs[count];
            count += 1;
        }

        count as i32
    }
}

fn main() {
    let result = Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 0);
    println!("{}", result);
}
