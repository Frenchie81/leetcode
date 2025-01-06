// problem 55
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut max_index = 0;
        for i in 0..nums.len() {
            if i > max_index {
                return false;
            }

            let num = nums[i];
            let max_from_here = i + num as usize;
            if max_from_here > max_index {
                max_index = max_from_here;
            }

            if max_index >= nums.len() - 1 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];

        let output = Solution::can_jump(nums);

        assert!(output);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 1, 0, 4];

        let output = Solution::can_jump(nums);

        assert!(!output);
    }
}
