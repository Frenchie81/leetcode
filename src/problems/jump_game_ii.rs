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
        let mut jumps = 0;
        let mut i = 0;

        while i < target {
            let val = nums[i];
            let max_from_here = i + val as usize;
            if max_from_here >= target {
                jumps += 1;
                return jumps;
            }

            let mut next_max_jump = max_from_here;
            let mut next_i = i;
            (i..=max_from_here).for_each(|j| {
                let val2 = nums[j];
                let next_max = j + val2 as usize;
                if next_max > next_max_jump {
                    next_max_jump = next_max;
                    next_i = j;
                }
            });

            jumps += 1;
            i = next_i;
        }

        jumps
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
