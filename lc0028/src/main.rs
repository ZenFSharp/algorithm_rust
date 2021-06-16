struct Solution{}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let result = haystack.find(&needle);
        if result.is_none() {return -1}
        result.unwrap() as i32
    }
}

fn main() {
    let result = Solution::str_str("hello".to_owned(), "ll".to_owned());
    println!("{}",result);
}
