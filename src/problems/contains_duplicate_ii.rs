use std::collections::HashMap;

// problem 219
struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let i = i as i32;
            if let Some(j) = hash.get(num) {
                let j = *j;
                if (i - j).abs() <= k {
                    return true;
                }
            }

            hash.insert(*num, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;

        let output = Solution::contains_nearby_duplicate(nums, k);

        assert!(output);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;

        let output = Solution::contains_nearby_duplicate(nums, k);

        assert!(output);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;

        let output = Solution::contains_nearby_duplicate(nums, k);

        assert!(!output);
    }
}
