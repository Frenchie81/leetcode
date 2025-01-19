// problem 9
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x: Vec<char> = x.to_string().chars().collect();
        for i in 0..x.len() {
            let j = x.len() - (i + 1);
            if i > j {
                break;
            }
            if x[i] != x[j] {
                return false;
            }
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
}
