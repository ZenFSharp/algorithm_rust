
struct Solution{}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut strings = Vec::new();
        for _ in 0..num_rows {
            strings.push(String::new());
        }
        let mut p = 0_i32;
        for c in s.chars() {
            strings[p.abs() as usize].push(c);
            p = if p == num_rows - 1 { 1 - p } else { p + 1 }
        }

        let mut result = String::new();
        for s in strings{result.push_str(&s)}
        result
    }
}

fn main() {
    let result = Solution::convert("PAYPALISHIRING".to_string(), 3);
    println!("{}",result);
}
