// problem 167
struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_i = 0;
        let mut right_i = numbers.len() - 1;

        while left_i < right_i {
            let left = numbers[left_i];
            let right = numbers[right_i];

            let sum = left + right;
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => left_i += 1,
                std::cmp::Ordering::Equal => return vec![left_i as i32 + 1, right_i as i32 + 1],
                std::cmp::Ordering::Greater => right_i -= 1,
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;

        let output = Solution::two_sum(numbers, target);

        assert_eq!(vec![1, 2], output);
    }

    #[test]
    fn example2() {
        let numbers = vec![2, 3, 4];
        let target = 6;

        let output = Solution::two_sum(numbers, target);

        assert_eq!(vec![1, 3], output);
    }
    #[test]
    fn example3() {
        let numbers = vec![-1, 0];
        let target = -1;

        let output = Solution::two_sum(numbers, target);

        assert_eq!(vec![1, 2], output);
    }

    #[test]
    fn my_test1() {
        let nums = vec![2, 5, 8, 10, 20, 26, 30];
        let target = 34;

        let output = Solution::two_sum(nums, target);

        assert_eq!(vec![3, 6], output);
    }
}
