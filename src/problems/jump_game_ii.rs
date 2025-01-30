// problem 45
struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return 0;
        }

        let target = nums.len() - 1;
        let mut max_index = 0;
        let mut jumps = 0;

        for (x, val) in nums.iter().enumerate() {
            let max_from_here = x + *val as usize;
            if max_from_here > max_index {
                max_index = max_from_here;
                jumps += 1;
            }

            if max_index >= target {
                return jumps;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 3, 1, 1, 4];

        let output = Solution::jump(nums);

        assert_eq!(2, output);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 3, 0, 1, 4];

        let output = Solution::jump(nums);

        assert_eq!(2, output);
    }

    #[test]
    fn wrong_answer1() {
        let nums = vec![1];

        let output = Solution::jump(nums);

        assert_eq!(0, output);
    }

    #[test]
    fn wrong_answer2() {
        let nums = vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3];

        let output = Solution::jump(nums);

        assert_eq!(2, output);
    }
}
