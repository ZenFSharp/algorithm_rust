struct Solution{}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        let mut words = s.trim().split(' ');
        while let Some(w) = words.next(){
            length = w.len();
        }
        length as i32
    }
}

fn main() {
    let result = Solution::length_of_last_word(String::from(" "));
    println!("{}",result);
}
