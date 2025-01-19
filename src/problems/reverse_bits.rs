// problem 190
struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut rev = 0;

        for i in 0..32 {
            let x = (x >> i) & 1;
            rev |= x << (31 - i);
        }

        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 43261596;

        let output = Solution::reverse_bits(n);

        assert_eq!(964176192, output);
    }

    #[test]
    fn example2() {
        let n = 4294967293;

        let output = Solution::reverse_bits(n);

        assert_eq!(3221225471, output);
    }
}
