// problem 14
struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::new();
        let strs_chars = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        if let Some(first_word) = strs_chars.first() {
            for (i, cur) in first_word.iter().enumerate() {
                for next_word_index in 1..strs_chars.len() {
                    let next_word = &strs_chars[next_word_index];
                    if let Some(next) = next_word.get(i) {
                        if next != cur {
                            return prefix;
                        }
                    } else {
                        return prefix;
                    }
                }
                prefix.push(*cur);
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        let output = Solution::longest_common_prefix(input);

        assert_eq!("fl", output);
    }

    #[test]
    fn example2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        let output = Solution::longest_common_prefix(input);

        assert_eq!("", output);
    }
}
