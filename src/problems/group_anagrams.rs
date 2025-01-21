use std::collections::HashMap;

// problem 49
struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map: HashMap<Vec<u32>, Vec<String>> = HashMap::new();

        for str in strs {
            let mut count = vec![0; 26];

            for c in str.chars() {
                let index = (c as u8) - b'a';
                count[index as usize] += 1;
            }

            if let Some(existing) = hash_map.get_mut(&count) {
                existing.push(str);
            } else {
                hash_map.insert(count, vec![str]);
            }
        }

        hash_map.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example1() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let mut output = Solution::group_anagrams(strs);
        for o in output.iter_mut() {
            o.sort();
        }
        output.sort();

        let mut expected = vec![
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        ];

        for e in expected.iter_mut() {
            e.sort();
        }
        expected.sort();

        assert_eq!(expected, output);
    }

    #[test]
    fn example2() {
        let strs = vec!["".to_string()];

        let output = Solution::group_anagrams(strs);

        let expected = vec![vec!["".to_string()]];
        assert_eq!(expected, output);
    }

    #[test]
    fn example3() {
        let strs = vec!["a".to_string()];

        let output = Solution::group_anagrams(strs);

        let expected = vec![vec!["a".to_string()]];
        assert_eq!(expected, output);
    }
}
