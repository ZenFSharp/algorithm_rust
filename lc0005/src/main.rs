struct Solution{}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return String::new();
        }
        let mut start = 0;
        let mut end = 0;

        let mut v = Vec::new();
        for c in s.chars() {
            v.push(c)
        }

        for i in 0..s.len() {
            let (l1, r1) = expand_around_center(&v, i as i32, i as i32);
            let (l2, r2) = expand_around_center(&v, i as i32, (i + 1) as i32);

            if r1 - l1 > end - start {
                start = l1;
                end = r1;
            }
            if r2 - l2 > end - start {
                start = l2;
                end = r2;
            }
        }

        s[start as usize..end as usize + 1].to_string()
    }
}

fn expand_around_center(s: &Vec<char>, left: i32, right: i32) -> (i32, i32) {
    let mut left = left;
    let mut right = right;
    while left >= 0 && (right as usize) < s.len() && s[left as usize] == s[right as usize] {
        left -= 1;
        right += 1;
    }
    (left + 1, right - 1)
}

fn main() {
    let result = Solution::longest_palindrome("adbbac".to_string());
    println!("{}",result);
}
