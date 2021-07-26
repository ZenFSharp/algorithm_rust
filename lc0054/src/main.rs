struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut x_start = 0;
        let mut x_end = matrix[0].len() - 1;
        let mut y_start = 0;
        let mut y_end = matrix.len() - 1;
        let mut result = Vec::with_capacity(matrix[0].len() * matrix.len());

        while x_start <= x_end && y_start <= y_end {
            let mut x = x_start;
            let mut y = y_start;
            while x < x_end {
                result.push(matrix[y][x]);
                x += 1;
            }
            while y < y_end {
                result.push(matrix[y][x]);
                y += 1;
            }
            if x_end == x_start || y_end == y_start {
                result.push(matrix[y][x]);
                break;
            }
            while x > x_start {
                result.push(matrix[y][x]);
                x -= 1;
            }
            while y > y_start {
                result.push(matrix[y][x]);
                y -= 1;
            }
            x_start += 1;
            x_end -= 1;
            y_start += 1;
            y_end -= 1;
        }
        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ])
    );
}
