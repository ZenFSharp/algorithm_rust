struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for c in s.chars() {
            match c {
                '{' => v.push(1),
                '[' => v.push(2),
                '(' => v.push(3),
                '}' => {
                    if v.pop() != Some(1) {
                        return false;
                    }
                }
                ']' => {
                    if v.pop() != Some(2) {
                        return false;
                    }
                }
                ')' => {
                    if v.pop() != Some(3) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        v.pop().is_none()
    }
}

fn main() {
    let result = Solution::is_valid("{[]}".to_owned());
    println!("{}", result);
}
