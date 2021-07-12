struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sets = vec![
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];
        let mut i = 0;
        let mut j = 0;
        while i < 9 {
            while j < 9 {
                if board[i][j] != '.' {
                    if !sets[i].insert(board[i][j]) {
                        return false;
                    }
                }
                j += 1
            }
            j = 0;
            i += 1
        }

        for s in sets.iter_mut() {
            s.clear();
        }
        i = 0;
        j = 0;
        while i < 9 {
            while j < 9 {
                if board[i][j] != '.' {
                    if !sets[j].insert(board[i][j]) {
                        return false;
                    }
                }
                j += 1
            }
            j = 0;
            i += 1
        }

        for s in sets.iter_mut() {
            s.clear();
        }
        i = 0;
        j = 0;
        while i < 9 {
            while j < 9 {
                if board[i][j] != '.' {
                    if !sets[i/3*3+j/3].insert(board[i][j]) {
                        return false;
                    }
                }
                j += 1
            }
            j = 0;
            i += 1
        }

        true
    }
}

fn main() {
    let result = Solution::is_valid_sudoku(vec![
        vec!['9', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
    println!("{:?}", result);
}
