
struct Solution{}
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let str = str.trim();
        let mut sign = false;
        let mut vc = Vec::new();
        for c in str.chars() {
            vc.push(c);
        }
        if vc.len()==0 {return 0;}
        let mut result = 0_i32;
        match vc[0] {
            '+' => {
                vc.remove(0);
            }
            '-' => {
                vc.remove(0);
                sign = true;
            }
            '0'..='9' => (),
            _ => return 0,
        }

        let mut i = 0;
        while i < vc.len() {
            match vc[i] {
                '0'..='9' => {
                    let n = vc[i].to_digit(10).unwrap() as i32;
                    match result.checked_mul(10) {
                        Some(r) => result = r,
                        None => {
                            return if sign {
                                i32::min_value()
                            } else {
                                i32::max_value()
                            };
                        }
                    }
                    match result.checked_add(n) {
                        Some(r) => result = r,
                        None => {
                            return if sign {
                                i32::min_value()
                            } else {
                                i32::max_value()
                            };
                        }
                    }
                }
                _ => return 0,
            }
            i += 1;
        }

        if sign {
            0 - result
        } else {
            result
        }
    }
}

fn main() {
    let result = Solution::my_atoi("-91283472332".to_string());
    println!("{}",result);
}
