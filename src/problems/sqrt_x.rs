// problem 69
struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut upper = x;
        let mut lower = 1;

        while upper - lower > 1 {
            let guess = (lower as i64 + upper as i64) / 2;
            let squared = guess.saturating_mul(guess);

            match squared.cmp(&(x as i64)) {
                std::cmp::Ordering::Less => lower = guess as i32,
                std::cmp::Ordering::Equal => return guess as i32,
                std::cmp::Ordering::Greater => upper = guess as i32,
            }
        }

        lower
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let x = 4;

        let output = Solution::my_sqrt(x);

        assert_eq!(2, output);
    }

    #[test]
    fn example2() {
        let x = 8;

        let output = Solution::my_sqrt(x);

        assert_eq!(2, output);
    }

    #[test]
    fn wrong_answer1() {
        let x = 2147395599;

        let output = Solution::my_sqrt(x);

        assert_eq!(46339, output);
    }

    #[test]
    fn wrong_answer2() {
        let x = 1;

        let output = Solution::my_sqrt(x);

        assert_eq!(1, output);
    }

    #[test]
    fn wrong_answer3() {
        let x = 2147483647;

        let output = Solution::my_sqrt(x);

        assert_eq!(46340, output);
    }
}
