// problem 189
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        Solution::recurse(nums, 0, k);
    }

    fn recurse(nums: &mut [i32], cur_index: usize, k: i32) {
        let len = nums.len();
        if cur_index >= len {
            return;
        }

        let mut new_index = cur_index + k as usize;
        if new_index > (len - 1) {
            new_index %= len;
        }

        let num = nums[cur_index];

        Solution::recurse(nums, cur_index + 1, k);

        nums[new_index] = num;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;

        Solution::rotate(&mut nums, k);

        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);
    }

    #[test]
    fn example2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;

        Solution::rotate(&mut nums, k);

        assert_eq!(vec![3, 99, -1, -100], nums);
    }
}
