use std::collections::HashMap;

// problem 70
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = HashMap::new();

        Solution::climb_stairs_cached(n, &mut cache)
    }

    fn climb_stairs_cached(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if cache.contains_key(&n) {
            return *cache.get(&n).unwrap();
        }

        if n <= 1 {
            return 1;
        }

        let result = Solution::climb_stairs_cached(n - 1, cache)
            + Solution::climb_stairs_cached(n - 2, cache);
        cache.insert(n, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 2;

        let output = Solution::climb_stairs(n);

        assert_eq!(2, output);
    }

    #[test]
    fn example2() {
        let n = 3;

        let output = Solution::climb_stairs(n);

        assert_eq!(3, output);
    }
}
