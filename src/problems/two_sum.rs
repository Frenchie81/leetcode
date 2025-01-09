use std::collections::HashMap;

// problem 1
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for (i, val) in nums.iter().enumerate() {
            let diff = target - val;
            if let Some(diff_index) = hash.get(&diff) {
                return vec![*diff_index, i as i32];
            } else {
                hash.insert(val, i as i32);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let output = Solution::two_sum(nums, target);

        assert_eq!(vec![0, 1], output);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let output = Solution::two_sum(nums, target);

        assert_eq!(vec![1, 2], output);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;

        let output = Solution::two_sum(nums, target);

        assert_eq!(vec![0, 1], output);
    }

    #[test]
    fn wrong_answer1() {
        let nums = vec![-3, 4, 3, 90];
        let target = 0;

        let output = Solution::two_sum(nums, target);

        assert_eq!(vec![0, 2], output);
    }
}
