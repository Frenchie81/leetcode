// problem 28
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        for (i, window) in haystack.windows(needle.len()).enumerate() {
            if window == needle {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();

        let output = Solution::str_str(haystack, needle);

        assert_eq!(0, output);
    }

    #[test]
    fn example2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();

        let output = Solution::str_str(haystack, needle);

        assert_eq!(-1, output);
    }
}
