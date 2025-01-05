// problem 27
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let len = nums.len();
        let mut k = 0;

        for i in (0..len).rev() {
            let cur = nums[i];
            if val == cur {
                k += 1;
                // shift everything left
                for j in i..len - k {
                    nums[j] = nums[j + 1];
                }
            }
        }

        (len - k) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;

        let k = Solution::remove_element(&mut nums, val) as usize;

        let mut slice = nums[0..k].to_vec();
        slice.sort();

        assert_eq!(vec![2, 2], slice);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;

        let k = Solution::remove_element(&mut nums, val) as usize;

        let mut slice = nums[0..k].to_vec();
        slice.sort();

        assert_eq!(vec![0, 0, 1, 3, 4], slice);
    }
}
