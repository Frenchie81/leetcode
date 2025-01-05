// problem 88
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
        if n <= 0 {
            return;
        }

        let mut insert_at = nums1.len() - 1;
        let mut m = m;

        for i2 in (0..n as usize).rev() {
            let val2 = nums2[i2];

            let mut inserted = false;
            for i1 in (0..m as usize).rev() {
                let val1 = nums1[i1];

                if val2 > val1 {
                    nums1[insert_at] = val2;
                    insert_at = insert_at.saturating_sub(1);
                    inserted = true;
                    break;
                } else {
                    nums1[insert_at] = val1;
                    insert_at = insert_at.saturating_sub(1);
                    m -= 1;
                }
            }

            if !inserted {
                nums1[insert_at] = val2;
                insert_at = insert_at.saturating_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let expected = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn example2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let expected = vec![1];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn example3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let expected = vec![1];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn wrong_answer1() {
        let mut nums1 = vec![2, 0];
        let m = 1;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let expected = vec![1, 2];
        assert_eq!(expected, nums1);
    }

    #[test]
    fn wrong_answer2() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![1, 2, 3];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        let expected = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(expected, nums1);
    }
}
