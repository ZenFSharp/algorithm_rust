struct Solution {}
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut i = 0;
        while i < deliciousness.len() {
            let mut j = i + 1;
            while j < deliciousness.len() {
                if good_meal(deliciousness[i] + deliciousness[j]) {
                    count += 1
                }
                j += 1
            }
            i += 1
        }
        count
    }
}

fn good_meal(num: i32) -> bool {
    num > 0 && num ^ (num - 1) == num * 2 - 1
}

fn main() {
    let result = Solution::count_pairs(vec![
        2160, 1936, 3, 29, 27, 5, 2503, 1593, 2, 0, 16, 0, 3860, 28908, 6, 2, 15, 49, 6246, 1946,
        23, 105, 7996, 196, 0, 2, 55, 457, 5, 3, 924, 7268, 16, 48, 4, 0, 12, 116, 2628, 1468,
    ]);
    println!("{}", result);
}
