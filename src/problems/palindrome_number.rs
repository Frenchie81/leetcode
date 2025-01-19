// problem 9
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x = x as i64;
        let mut div = 1_i64;
        while x >= 10_i64 * div {
            div *= 10;
        }

        while x > 0 {
            let right = x % 10;
            let left = x / div;

            if left != right {
                return false;
            }

            x = (x % div) / 10;
            div /= 100;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let x = 121;

        let output = Solution::is_palindrome(x);

        assert!(output);
    }

    #[test]
    fn example2() {
        let x = -121;

        let output = Solution::is_palindrome(x);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let x = 10;

        let output = Solution::is_palindrome(x);

        assert!(!output);
    }

    #[test]
    fn wrong_answer1() {
        let x = 1410110141;

        let output = Solution::is_palindrome(x);

        assert!(output);
    }
}
