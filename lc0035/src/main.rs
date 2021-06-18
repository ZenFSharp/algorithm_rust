struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut pre = 0;
        let mut post = len;
        let mut p = len / 2;
        while p > pre && p < post {
            match p {
                _ if nums.get(p).unwrap() == &target => break,
                _ if nums.get(p).unwrap() < &target => {
                    pre = p;
                    p = (p + post) / 2;
                }
                _ => {
                    post = p;
                    p = (p + pre) / 2;
                }
            }
        }

        if nums.get(p).unwrap() < &target {
            p += 1
        }

        p as i32
    }
}

fn main() {
    let result = Solution::search_insert(vec![1, 3, 5, 6], 4);
    println!("{}", result);
}
