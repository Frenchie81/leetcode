use std::collections::HashSet;

// problem 202
struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut hash = HashSet::new();
        while n != 1 {
            n = n
                .to_string()
                .chars()
                .map(|c| {
                    let x = c.to_digit(10).unwrap();
                    x * x
                })
                .sum::<u32>() as i32;
            if hash.contains(&n) {
                return false;
            }
            hash.insert(n);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 19;

        let output = Solution::is_happy(n);

        assert!(output);
    }

    #[test]
    fn example2() {
        let n = 2;

        let output = Solution::is_happy(n);

        assert!(!output);
    }
}
