struct Solution {}
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut markers = [false; 20002];
        let mut max = 0;
        for interval in intervals {
            let mut i = interval[0] * 2;
            while i < interval[1] * 2 {
                max = std::cmp::max(max, i);
                if !markers[i as usize] {
                    markers[i as usize] = true
                }
                i += 1
            }
            markers[interval[1] as usize * 2] = true;
            max = std::cmp::max(max, interval[1] * 2);
        }
        let mut result = vec![];
        let mut start = 0;
        let mut end;
        let mut is_start = true;
        let mut i = 0;
        while i <= max as usize {
            match is_start {
                true => {
                    if markers[i] {
                        start = i;
                        is_start = false;
                    }
                }
                false => {
                    if !markers[i] {
                        end = i;
                        is_start = true;
                        result.push(vec![start as i32 / 2, end as i32 / 2])
                    }
                }
            }
            i += 1
        }
        result.push(vec![start as i32/2, max/2]);
        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::merge(vec![vec![2,3],vec![5,5],vec![2,2],vec![3,4],vec![3,4]])
    );
}
