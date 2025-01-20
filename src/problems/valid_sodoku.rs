use std::collections::{HashMap, HashSet};

// problem 36
struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut column_hashsets: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut sub_box_hashsets: HashMap<(usize, usize), HashSet<char>> = HashMap::new();
        for (y, row) in board.iter().enumerate() {
            let mut row_hashset = HashSet::new();
            for (x, cell) in row.iter().enumerate() {
                if *cell == '.' {
                    continue;
                }

                let sub_box_hashset = sub_box_hashsets.entry((x / 3, y / 3)).or_default();
                if sub_box_hashset.contains(cell) {
                    return false;
                }

                let column_hashset = column_hashsets.entry(x).or_default();
                if column_hashset.contains(cell) {
                    return false;
                }

                if row_hashset.contains(cell) {
                    return false;
                }

                sub_box_hashset.insert(*cell);
                row_hashset.insert(*cell);
                column_hashset.insert(*cell);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let output = Solution::is_valid_sudoku(board);

        assert!(output);
    }

    #[test]
    fn example2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let output = Solution::is_valid_sudoku(board);

        assert!(!output);
    }
}
