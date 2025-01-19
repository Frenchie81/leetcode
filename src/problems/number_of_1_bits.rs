// problem 191
struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut count = 0;

        for i in 0..32 {
            let bit = (n >> i) & 1;
            count += bit;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 11;

        let output = Solution::hamming_weight(n);

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let n = 128;

        let output = Solution::hamming_weight(n);

        assert_eq!(1, output);
    }

    #[test]
    fn example3() {
        let n = 2147483645;

        let output = Solution::hamming_weight(n);

        assert_eq!(30, output);
    }
}
