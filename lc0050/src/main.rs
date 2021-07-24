struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u32; 26], Vec<String>> = HashMap::new();
        for str in strs.into_iter(){
            let feature = get_feature(&str);
            let v = map.entry(feature).or_insert(vec![]);
            v.push(str);
        }
        map.into_iter().map(|(_,s)|s).collect::<Vec<_>>()
    }
}
fn get_feature(word: &str) -> [u32; 26] {
    let mut feature = [0; 26];
    for c in word.chars() {
        let i = c as u8 - 'a' as u8;
        feature[i as usize] += 1;
    }
    feature
}
fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ])
    );
}
