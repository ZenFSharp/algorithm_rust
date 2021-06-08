struct Solution{}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        let mut cache = 0;
        let mut d;
        for c in s.chars() {
            d = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 1001,
            };
            n += if cache < d { 0 - cache } else { cache };
            cache = d;
        }
        n += cache;
        n
    }
}
fn main() {
    let result = Solution::roman_to_int("LVIII".to_owned());
    println!("{}",result);
}
