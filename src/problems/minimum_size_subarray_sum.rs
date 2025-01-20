// problem 209
struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut min_subarray = None;

        let mut i_start = 0;
        let mut i_end = 0;
        let mut current_sum = nums[0];

        while i_start < nums.len() {
            if current_sum >= target {
                let window = (i_end - i_start) as i32 + 1;
                if window == 1 {
                    return 1;
                }

                if let Some(min) = min_subarray {
                    if window < min {
                        min_subarray = Some(window);
                    }
                } else {
                    min_subarray = Some(window);
                }

                let start_val = nums[i_start];
                current_sum -= start_val;
                i_start += 1;
            } else if i_end < nums.len() - 1 {
                i_end += 1;
                let end_val = nums[i_end];
                current_sum += end_val;
            } else {
                break;
            }
        }

        min_subarray.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];

        let output = Solution::min_sub_array_len(target, nums);

        assert_eq!(2, output)
    }

    #[test]
    fn example2() {
        let target = 4;
        let nums = vec![1, 4, 4];

        let output = Solution::min_sub_array_len(target, nums);

        assert_eq!(1, output)
    }

    #[test]
    fn example3() {
        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];

        let output = Solution::min_sub_array_len(target, nums);

        assert_eq!(0, output)
    }

    #[test]
    fn wrong_answer1() {
        let target = 11;
        let nums = vec![1, 2, 3, 4, 5];

        let output = Solution::min_sub_array_len(target, nums);

        assert_eq!(3, output)
    }
}
