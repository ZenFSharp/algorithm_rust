struct Solution{}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = x >= 0;
        let mut x = x.abs();
        let mut v = Vec::new();
        while x > 0 {
            v.push(x % 10);
            x /= 10;
        }
        let mut result = 0_i32;
        let mut i = 0;
        while i < v.len() {
            match result.checked_mul(10) {
                Some(r) => result = r,
                None => {
                    result = 0;
                    break;
                }
            }
            match result.checked_add(v[i]) {
                Some(r) => result = r,
                None => {
                    result = 0;
                    break;
                }
            }
            i += 1;
        }
        if sign {
            result
        } else {
            0 - result
        }
    }
}

fn main() {
    let result = Solution::reverse(-123);
    println!("{}",result);
}
