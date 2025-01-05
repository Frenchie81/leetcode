use std::collections::HashMap;

// problem 169
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }
        let majority = counts.iter().max_by(|a, b| a.1.cmp(b.1));
        *majority.unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3, 2, 3];

        let output = Solution::majority_element(nums);

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];

        let output = Solution::majority_element(nums);

        assert_eq!(2, output);
    }
}
