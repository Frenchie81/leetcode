use std::collections::HashMap;

// problem 383
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut magazine_map: HashMap<char, u32> = HashMap::new();
        for m in magazine.chars() {
            magazine_map.entry(m).and_modify(|c| *c += 1).or_insert(1);
        }

        for r in ransom_note.chars() {
            if let Some(m) = magazine_map.get_mut(&r) {
                if *m == 0 {
                    return false;
                }
                *m -= 1;
            } else {
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
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();

        let output = Solution::can_construct(ransom_note, magazine);

        assert!(!output);
    }

    #[test]
    fn example2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();

        let output = Solution::can_construct(ransom_note, magazine);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();

        let output = Solution::can_construct(ransom_note, magazine);

        assert!(output);
    }
}
