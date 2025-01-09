use std::collections::HashMap;

// problem 205
struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let mut hash: HashMap<char, char> = HashMap::new();
        for i in 0..s.len() {
            let s_char = s[i];
            let t_char = t[i];

            if let Some(mapped_t_char) = hash.get(&s_char) {
                if *mapped_t_char != t_char {
                    return false;
                }
            } else if hash.values().any(|v| *v == t_char) {
                // we have already mapped this char
                return false;
            } else {
                hash.insert(s_char, t_char);
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
        let s = "egg".to_string();
        let t = "add".to_string();

        let output = Solution::is_isomorphic(s, t);

        assert!(output);
    }

    #[test]
    fn example2() {
        let s = "foo".to_string();
        let t = "bar".to_string();

        let output = Solution::is_isomorphic(s, t);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let s = "paper".to_string();
        let t = "title".to_string();

        let output = Solution::is_isomorphic(s, t);

        assert!(output);
    }

    #[test]
    fn wrong_answer1() {
        let s = "badc".to_string();
        let t = "baba".to_string();

        let output = Solution::is_isomorphic(s, t);

        assert!(!output);
    }
}
