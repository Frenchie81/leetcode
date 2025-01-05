// problem 26

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut k = 0;
        let len = nums.len();
        let mut insert_at = 1;

        let mut i = 1;
        while i < len {
            let prev = nums[i - 1];
            let cur = nums[i];
            if prev == cur {
                k += 1;
            } else {
                nums[insert_at] = cur;
                insert_at += 1;
            }

            i += 1;
        }

        (len - k) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1, 1, 2];

        let k = Solution::remove_duplicates(&mut nums) as usize;

        assert_eq!([1, 2], &nums[0..k]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let k = Solution::remove_duplicates(&mut nums) as usize;

        assert_eq!([0, 1, 2, 3, 4], &nums[0..k]);
    }
}
