struct Solution{}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let l = strs.len();
        if l ==0 {return String::from("")}
        let mut result = Vec::new();
        let bytes: Vec<Vec<u8>> = strs
            .iter()
            .map(|s| s.clone().into_bytes())
            .collect();

        let mut i = 0;
        let mut done = false;
        loop {
            match bytes[0].get(i) {
                None => break,
                Some(b) => {
                    let mut j = 1;
                    while j < l {
                        match bytes[j].get(i) {
                            None => {
                                done = true;
                                break;
                            }
                            Some(_b) => {
                                if _b != b {
                                    done = true;
                                    break;
                                }
                            }
                        }
                        j += 1;
                    }
                    if done {
                        break;
                    }
                    result.push(b.clone());
                }
            }
            i += 1;
        }

        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let result = Solution::longest_common_prefix(vec!("flower".to_owned(),"flow".to_owned(),"flight".to_owned()));
    println!("{}",result);
}
