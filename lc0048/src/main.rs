struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let mut x = 0;
        let mut y:usize;

        while x < len / 2 {
            y = 0;
            while y < len {
                let tmp = matrix[x][y];
                matrix[x][y] = matrix[len - x - 1][y];
                matrix[len - x - 1][y] = tmp;
                y += 1;
            }
            x += 1;
        }

        x = 0;
        while x < len {
            y = 0;
            while y < x {
                let tmp = matrix[x][y];
                matrix[x][y] = matrix[y][x];
                matrix[y][x] = tmp;
                y += 1
            }
            x += 1
        }
    }
}

fn main() {
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}
