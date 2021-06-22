struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let mut num = num;
        while num >= 1000 {
            result.push('M');
            num -= 1000;
        }
        match num {
            n if n >= 900 => {
                result.push('C');
                result.push('M');
                num -= 900;
            }
            n if n >= 500 => {
                result.push('D');
                num -= 500;
            }
            n if n >= 400 => {
                result.push('C');
                result.push('D');
                num -= 400;
            }
            _ => {}
        }
        while num >=100{
            result.push('C');
            num -= 100;
        }
        match num {
            n if n >= 90 => {
                result.push('X');
                result.push('C');
                num -= 90;
            }
            n if n >= 50 => {
                result.push('L');
                num -= 50;
            }
            n if n >= 40 => {
                result.push('X');
                result.push('L');
                num -= 40;
            }
            _ => {}
        }
        while num >=10{
            result.push('X');
            num -= 10;
        }
        match num {
            n if n >= 9 => {
                result.push('I');
                result.push('X');
                num -= 9;
            }
            n if n >= 5 => {
                result.push('V');
                num -= 5;
            }
            n if n >= 4 => {
                result.push('I');
                result.push('V');
                num -= 4;
            }
            _ => {}
        }
        while num >=1{
            result.push('I');
            num -= 1;
        }
        result
    }
}

fn main() {
    let result = Solution::int_to_roman(1994);
    println!("{}", result);
}
