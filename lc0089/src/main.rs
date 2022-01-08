struct Solution {}
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(1 << n);
        result.push(0);
        result.push(1);
        for i in 1..n {
            let mut rev = result
                .iter()
                .map(|num| num + (1 << i))
                .rev()
                .collect::<Vec<_>>();
            result.append(&mut rev)
        }

        result
    }
}
fn main() {
    println!("{:?}", Solution::gray_code(16));
}
