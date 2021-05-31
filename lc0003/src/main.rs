struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let mut max = 0;
        let mut _max = 0;
        let mut _s = String::new();
        let mut hash: HashSet<char> = HashSet::new();

        for c in s.chars() {
            if hash.contains(&c) {
                loop{
                    let _c = _s.remove(0);
                    if _c!=c {hash.remove(&_c);} else{break;}
                }
                _s.push(c);
                _max = _s.len();
            } else {
                _max = _max + 1;
                hash.insert(c);
                _s.push(c);
            }
            if max<_max {max = _max;}
        }

        max as i32
    }
}
fn main() {
    let l = Solution::length_of_longest_substring(String::from("abba"));
    println!("{}", l)
}
