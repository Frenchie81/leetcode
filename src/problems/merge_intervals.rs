// problem 56
struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }

        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut results = vec![];

        let current = intervals.first().unwrap();
        let mut start = current[0];
        let mut end = current[1];
        for i in 1..intervals.len() {
            let next = intervals.get(i).unwrap();
            if next[0] < start || end >= next[0] {
                start = start.min(next[0]);
                end = end.max(next[1]);
            } else {
                results.push(vec![start, end]);
                start = next[0];
                end = next[1];
            }
        }

        results.push(vec![start, end]);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];

        let output = Solution::merge(intervals);

        assert_eq!(vec![vec![1, 6], vec![8, 10], vec![15, 18]], output);
    }

    #[test]
    fn example2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];

        let output = Solution::merge(intervals);

        assert_eq!(vec![vec![1, 5]], output);
    }

    #[test]
    fn wrong_answer1() {
        let intervals = vec![vec![1, 4], vec![0, 4]];

        let output = Solution::merge(intervals);

        assert_eq!(vec![vec![0, 4]], output);
    }

    #[test]
    fn wrong_answer2() {
        let intervals = vec![vec![1, 4], vec![0, 0]];

        let output = Solution::merge(intervals);

        assert_eq!(vec![vec![0, 0], vec![1, 4]], output);
    }
}
