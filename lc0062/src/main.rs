struct Solution {}

impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        if n > m {
            std::mem::swap(&mut m, &mut n);
        }
        let t = my_factorial((m - 1 + n - 1) as u64, (m - 1) as u64);
        let s = my_factorial((n - 1) as u64, 0);
        println!("{}", t);
        println!("{}", s);
        println!("{}", t / s);
        (t / s) as i32
    }
}
fn my_factorial(n: u64, m: u64) -> u64 {
    let mut i = n;
    let mut r = 1u64;
    while i > m {
        r *= i;
        i -= 1;
    }
    r
}
fn main() {
    println!("{}", Solution::unique_paths(13, 23));
}
