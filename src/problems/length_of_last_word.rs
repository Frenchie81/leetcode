// problem 58
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;

        let chars: Vec<char> = s.chars().collect();
        for i in (0..chars.len()).rev() {
            let c = chars[i];
            if length == 0 && c.is_whitespace() {
                // haven't found a word yet
                continue;
            }

            if c.is_whitespace() {
                break;
            } else {
                length += 1;
            }
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Hello World".to_string();

        let output = Solution::length_of_last_word(input);

        assert_eq!(5, output);
    }

    #[test]
    fn example2() {
        let input = "   fly me   to   the moon  ".to_string();

        let output = Solution::length_of_last_word(input);

        assert_eq!(4, output);
    }

    #[test]
    fn example3() {
        let input = "luffy is still joyboy".to_string();

        let output = Solution::length_of_last_word(input);

        assert_eq!(6, output);
    }
}
