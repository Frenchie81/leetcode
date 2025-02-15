// problem 238
struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        Self::recurse(0, 1, &nums, &mut result);

        result
    }

    fn recurse(index: usize, prefix: i32, nums: &[i32], result: &mut [i32]) -> i32 {
        if index >= nums.len() {
            return 1;
        }

        let val = nums[index];

        let next_prefix = val * prefix;

        let suffix = Self::recurse(index + 1, next_prefix, nums, result);

        result[index] = prefix * suffix;

        suffix * val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4];

        let output = Solution::product_except_self(nums);

        assert_eq!(vec![24, 12, 8, 6], output);
    }

    #[test]
    fn example2() {
        let nums = vec![-1, 1, 0, -3, 3];

        let output = Solution::product_except_self(nums);

        assert_eq!(vec![0, 0, 9, 0, 0], output);
    }
}
