// problem 20
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            let is_open = matches!(c, '{' | '(' | '[');
            if is_open {
                stack.push(c);
            } else if let Some(last_open) = stack.pop() {
                if (c == '}' && last_open == '{')
                    || (c == ')' && last_open == '(')
                    || (c == ']' && last_open == '[')
                {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "()".to_string();

        let output = Solution::is_valid(s);

        assert!(output);
    }

    #[test]
    fn example2() {
        let s = "()[]{}".to_string();

        let output = Solution::is_valid(s);

        assert!(output);
    }

    #[test]
    fn example3() {
        let s = "(]".to_string();

        let output = Solution::is_valid(s);

        assert!(!output);
    }

    #[test]
    fn example4() {
        let s = "([])".to_string();

        let output = Solution::is_valid(s);

        assert!(output);
    }
}
