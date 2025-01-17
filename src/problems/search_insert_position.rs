// problem 35
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::recurse(&nums, target, 0, nums.len() - 1)
    }

    fn recurse(nums: &[i32], target: i32, i_start: usize, i_end: usize) -> i32 {
        let middle = (i_end + i_start) / 2;
        let val = nums[middle];
        if val == target {
            return middle as i32;
        }

        if target > val {
            if middle == i_end {
                return middle as i32 + 1;
            }
            Solution::recurse(nums, target, middle + 1, i_end)
        } else {
            if middle == i_start {
                return middle as i32;
            }
            Solution::recurse(nums, target, i_start, middle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        let output = Solution::search_insert(nums, target);

        assert_eq!(2, output);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;

        let output = Solution::search_insert(nums, target);

        assert_eq!(1, output);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;

        let output = Solution::search_insert(nums, target);

        assert_eq!(4, output);
    }
}
