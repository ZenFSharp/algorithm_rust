struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut s = s;
        let mut map = HashMap::<char, i32>::new();
        while let Some(c) = s.pop() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut map_vec: Vec<_> = map.iter().collect();
        map_vec.sort_by(|kv1, kv2| kv2.1.cmp(kv1.1));
        map_vec
            .iter()
            .map(|(k, v)| (0..**v).map(|_| *k).collect::<String>())
            .collect::<String>()
    }
}

fn main() {
    let result = Solution::frequency_sort(String::from("abcdbcdcdd"));
    println!("{}", result);
}
