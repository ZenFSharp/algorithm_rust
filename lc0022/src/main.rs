struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        generate("", n, 0, &mut result);
        result
    }
}

fn generate(r: &str, n: i32, used: i32, result: &mut Vec<String>) {
    if n == 0 {
        let mut used = used;
        let mut sr = r.to_string();
        while used > 0 {
            sr.push(')');
            used -= 1;
        }
        result.push(sr);
        return;
    }
    if used == 0 {
        generate(&(r.to_string() + "("), n - 1, 1, result);
    } else {
        generate(&(r.to_string() + "("), n - 1, used + 1, result);
        generate(&(r.to_string() + ")"), n, used - 1, result);
    }
}

fn main() {
    let result = Solution::generate_parenthesis(3);
    println!("{:?}", result);
}
