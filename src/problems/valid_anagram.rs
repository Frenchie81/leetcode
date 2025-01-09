use std::collections::HashMap;

// problem 242
struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s_hash = Self::count(s);
        let t_hash = Self::count(t);

        s_hash == t_hash
    }

    fn count(val: String) -> HashMap<char, u32> {
        let mut hash = HashMap::new();

        for c in val.chars() {
            hash.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        let output = Solution::is_anagram(s, t);

        assert!(output);
    }

    #[test]
    fn example2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        let output = Solution::is_anagram(s, t);

        assert!(!output);
    }
}
