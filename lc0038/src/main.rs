struct Solution {}
impl Solution {
    pub fn count_and_say(mut n: i32) -> String {
        let mut result = String::from("1");
        while n > 1 {            
            result = just_say(result);
            n -= 1;
        }
        result
    }
}

fn just_say(mut s: String) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut last = 'x';
    for c in s.chars() {
        if count == 0 {
            last = c;
            count = 1;
        }
        else if c == last {
            count += 1;
        }
        else {
            result.push(char::from_digit(count, 10).unwrap());
            result.push(last);
            count = 1;
            last = c;
        }
    }
    result.push(char::from_digit(count, 10).unwrap());
    result.push(s.pop().unwrap());

    result
}

fn main() {
    let result = Solution::count_and_say(4);
    println!("{}", result);
}
