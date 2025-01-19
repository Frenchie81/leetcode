// problem 66
struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut append_one = true;
        for digit in digits.iter_mut().rev() {
            let sum = *digit + 1;
            if sum > 9 {
                *digit = 0;
            } else {
                *digit = sum;
                append_one = false;
                break;
            }
        }

        if append_one {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example1() {
        let digits = vec![1, 2, 3];

        let output = Solution::plus_one(digits);

        assert_eq!(vec![1, 2, 4], output);
    }

    #[test]
    fn example2() {
        let digits = vec![4, 3, 2, 1];

        let output = Solution::plus_one(digits);

        assert_eq!(vec![4, 3, 2, 2], output);
    }

    #[test]
    fn example3() {
        let digits = vec![9];

        let output = Solution::plus_one(digits);

        assert_eq!(vec![1, 0], output);
    }
}
