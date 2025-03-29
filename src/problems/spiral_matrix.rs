use std::collections::HashSet;

struct Solution;

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut output = Vec::new();
        Solution::recurse(&matrix, 0, 0, Direction::Right, &mut visited, &mut output);
        output
    }

    fn recurse(
        matrix: &[Vec<i32>],
        x: i32,
        y: i32,
        direction: Direction,
        visited: &mut HashSet<(i32, i32)>,
        output: &mut Vec<i32>,
    ) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        let row = matrix.get(x as usize);
        if row.is_none() {
            return false;
        }

        let row = row.unwrap();
        let column = row.get(y as usize);
        if column.is_none() {
            return false;
        }

        if visited.contains(&(x, y)) {
            return false;
        }

        let value = column.unwrap();
        output.push(*value);

        visited.insert((x, y));

        match direction {
            Direction::Right => {
                if !Solution::recurse(matrix, x, y + 1, direction, visited, output) {
                    Solution::recurse(matrix, x + 1, y, direction.turn(), visited, output);
                }
            }
            Direction::Down => {
                if !Solution::recurse(matrix, x + 1, y, direction, visited, output) {
                    Solution::recurse(matrix, x, y - 1, direction.turn(), visited, output);
                }
            }
            Direction::Left => {
                if !Solution::recurse(matrix, x, y - 1, direction, visited, output) {
                    Solution::recurse(matrix, x - 1, y, direction.turn(), visited, output);
                }
            }
            Direction::Up => {
                if !Solution::recurse(matrix, x - 1, y, direction, visited, output) {
                    Solution::recurse(matrix, x, y + 1, direction.turn(), visited, output);
                }
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
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let output = Solution::spiral_order(matrix);

        assert_eq!(vec![1, 2, 3, 6, 9, 8, 7, 4, 5], output);
    }

    #[test]
    fn example2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

        let output = Solution::spiral_order(matrix);

        assert_eq!(vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7], output);
    }
}
