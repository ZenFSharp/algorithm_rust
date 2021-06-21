struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut end = (height.len() - 1) as i32;

        while start < end {
            let c_max = if height[start as usize] - height[end as usize] > 0 {
                height[end as usize] * (end - start)
            } else {
                height[start as usize] * (end - start)
            };
            if c_max>max {max = c_max}
            if height[start as usize] - height[end as usize] > 0{
                end-=1;
            }
            else{
                start+=1;
            }
        }
        
        max
    }
}

fn main() {
    let result = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("{}", result);
}
