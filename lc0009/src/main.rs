struct Solution{}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 10 && x >= 0 {
            return true;
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut x = x;
        let mut half = 0;
        while x > half {
            half = half * 10 + x % 10;
            x /= 10
        }
        x == half || x == half / 10
    }
}


fn main() {
    let result = Solution::is_palindrome(12321);
    println!("{}",result);
}
