// problem 125
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase().chars().collect();
        let mut front_index = Solution::get_next_alphanumeric_index(&s, 0, true);
        let mut back_index = Solution::get_next_alphanumeric_index(&s, s.len() - 1, false);

        while let Some(f) = front_index {
            if front_index >= back_index {
                break;
            }

            if let Some(b) = back_index {
                let front = s[f];
                let back = s[b];
                if front != back {
                    return false;
                }

                front_index = Solution::get_next_alphanumeric_index(&s, f + 1, true);
                back_index = Solution::get_next_alphanumeric_index(&s, b - 1, false);
            } else {
                break;
            }
        }

        true
    }

    fn get_next_alphanumeric_index(
        chars: &[char],
        cur_index: usize,
        forward: bool,
    ) -> Option<usize> {
        let mut next_index = cur_index;
        loop {
            if let Some(c) = chars.get(next_index) {
                if c.is_alphanumeric() {
                    return Some(next_index);
                }
            } else {
                return None;
            }

            if forward {
                next_index += 1;
            } else if next_index > 0 {
                next_index -= 1;
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "A man, a plan, a canal: Panama".to_string();

        let output = Solution::is_palindrome(input);

        assert!(output);
    }

    #[test]
    fn example2() {
        let input = "race a ca".to_string();

        let output = Solution::is_palindrome(input);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let input = " ".to_string();

        let output = Solution::is_palindrome(input);

        assert!(output);
    }

    #[test]
    fn wrong_answer1() {
        let input = "ab".to_string();

        let output = Solution::is_palindrome(input);

        assert!(!output);
    }
}
