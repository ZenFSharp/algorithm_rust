struct Solution {}
impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let start = intervals
            .iter()
            .map(|v| v[0])
            .collect::<Vec<i32>>()
            .binary_search(&new_interval[0]);
        let mut start = match start {
            Ok(s) => s,
            Err(s) => s,
        };
        if start > 0 && new_interval[0] <= intervals[start - 1][1] {
            new_interval[0] = intervals[start - 1][0];
            start -= 1;
        }

        let end = intervals
            .iter()
            .map(|v| v[1])
            .collect::<Vec<i32>>()
            .binary_search(&new_interval[1]);
        let mut end = match end {
            Ok(e) => e as i32,
            Err(e) => e as i32,
        };
        if (end as usize) < intervals.len() && new_interval[1] >= intervals[end as usize][0] {
            new_interval[1] = intervals[end as usize][1]
        } else {
            end -= 1;
        }

        let mut i = start as i32;
        while i <= end {
            intervals.remove(start);
            i += 1
        }
        intervals.insert(start, new_interval);
        intervals
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::insert(
            vec![vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
            vec![1, 2]
        )
    );
}
