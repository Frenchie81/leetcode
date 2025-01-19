// problem 136
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bits = 0;
        for num in nums {
            bits ^= num;
        }
        bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 2, 1];

        let output = Solution::single_number(nums);

        assert_eq!(1, output);
    }

    #[test]
    fn example2() {
        let nums = vec![4, 1, 2, 1, 2];

        let output = Solution::single_number(nums);

        assert_eq!(4, output);
    }

    #[test]
    fn example3() {
        let nums = vec![1];

        let output = Solution::single_number(nums);

        assert_eq!(1, output);
    }
}
