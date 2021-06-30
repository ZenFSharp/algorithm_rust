struct Solution {}
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            return if dividend == -2147483648 {
                2147483647
            } else {
                -dividend
            };
        }
        let s = (dividend > 0 && divisor < 0) || (dividend < 0 && divisor > 0);

        let dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let result = sd(dividend, divisor) as i32;

        if s {
            -result 
        } else {
            result
        }
    }
}
fn sd(dividend: i64, divisor: i64) -> i64 {
    if dividend < divisor {
        return 0;
    }

    let mut result = 1i64;
    let mut t_divisor = divisor;
    while dividend >= t_divisor {
        t_divisor <<= 1;
        result <<= 1;
    }
    (result>>1) + sd(dividend - (t_divisor >> 1), divisor)
}

fn main() {
    let result = Solution::divide(-2147483648, 2);
    println!("{}", result);
}
