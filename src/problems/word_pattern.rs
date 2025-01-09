use std::collections::HashMap;

// problem 290
struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern: Vec<char> = pattern.chars().collect();
        let s: Vec<&str> = s.split_whitespace().collect();

        if pattern.len() != s.len() {
            return false;
        }

        let mut pattern_hash: HashMap<char, &str> = HashMap::new();
        let mut s_hash: HashMap<&str, char> = HashMap::new();
        for i in 0..pattern.len() {
            let p = pattern[i];
            let s_word = s[i];
            if let Some(pattern_match) = pattern_hash.get(&p) {
                if *pattern_match != s_word {
                    return false;
                }
            } else if s_hash.contains_key(s_word) {
                return false;
            } else {
                pattern_hash.insert(p, s_word);
                s_hash.insert(s_word, p);
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
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();

        let output = Solution::word_pattern(pattern, s);

        assert!(output);
    }

    #[test]
    fn example2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();

        let output = Solution::word_pattern(pattern, s);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();

        let output = Solution::word_pattern(pattern, s);

        assert!(!output);
    }
}
