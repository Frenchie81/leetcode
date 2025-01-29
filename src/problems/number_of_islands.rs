// problem 200
struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let max_x = grid.first().unwrap().len();
        let max_y = grid.len();
        let mut islands: Vec<Vec<i32>> = vec![vec![0_i32; max_x]; max_y];

        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '0' {
                    continue;
                }

                if islands[y][x] > 0 {
                    continue;
                }

                result += 1;
                Self::discover_island(&grid, x, y, &mut islands, result);
            }
        }
        result
    }

    fn discover_island(
        grid: &[Vec<char>],
        x: usize,
        y: usize,
        islands: &mut [Vec<i32>],
        island_id: i32,
    ) {
        let row = grid.get(y);
        if row.is_none() {
            return;
        }

        let row = row.unwrap();
        let c = row.get(x);
        if c.is_none() {
            return;
        }

        let c = c.unwrap();
        if *c == '0' {
            return;
        }

        if islands[y][x] > 0 {
            return;
        }

        islands[y][x] = island_id;

        // left
        if x > 0 {
            Self::discover_island(grid, x - 1, y, islands, island_id);
        }

        // right
        Self::discover_island(grid, x + 1, y, islands, island_id);

        // up
        if y > 0 {
            Self::discover_island(grid, x, y - 1, islands, island_id);
        }

        // down
        Self::discover_island(grid, x, y + 1, islands, island_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        let output = Solution::num_islands(grid);

        assert_eq!(1, output);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        let output = Solution::num_islands(grid);

        assert_eq!(3, output);
    }

    #[test]
    fn wrong_answer1() {
        let grid = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];

        let output = Solution::num_islands(grid);

        assert_eq!(1, output);
    }
}
