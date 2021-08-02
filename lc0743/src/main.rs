struct Solution {}
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let inf = i32::MAX / 2;
        let mut g = (0..n)
            .map(|_| (0..n).map(|_| inf).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        for t in times {
            g[(t[0] - 1) as usize][(t[1] - 1) as usize] = t[2]
        }

        let mut dist = (0..n).map(|_| inf).collect::<Vec<i32>>();
        dist[k as usize - 1] = 0;
        let mut used = (0..n).map(|_| false).collect::<Vec<bool>>();

        let mut i = 0;
        while i < n {
            let mut x = -1;
            let mut y = 0;
            while y < n {
                if !used[y as usize] && (x == -1 || dist[y as usize] < dist[x as usize]) {
                    x = y
                }
                y += 1
            }
            used[x as usize] = true;
            let mut y = 0;
            while y < n {
                dist[y as usize] = std::cmp::min(
                    dist[y as usize],
                    dist[x as usize] + g[x as usize][y as usize],
                );
                y += 1
            }
            i += 1
        }

        let res = dist.into_iter().max().unwrap();
        if res == inf {
            -1
        } else {
            res
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2)
    );
}
