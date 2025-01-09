// problem 392
struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let mut s_pos = 0;
        for t_pos in 0..t.len() {
            if let Some(s_char) = s.get(s_pos) {
                if let Some(t_char) = t.get(t_pos) {
                    if s_char == t_char {
                        s_pos += 1;
                    }
                    if s_pos == s.len() {
                        return true;
                    }
                }
            }
        }

        s_pos == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();

        let output = Solution::is_subsequence(s, t);

        assert!(output);
    }

    #[test]
    fn example2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();

        let output = Solution::is_subsequence(s, t);

        assert!(!output);
    }
}
