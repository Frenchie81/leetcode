// problem 67
struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a: Vec<char> = a.chars().collect();
        let mut b: Vec<char> = b.chars().collect();

        let mut output = String::new();

        Solution::recurse(&mut a, &mut b, 0, &mut output);

        output.chars().rev().collect()
    }

    fn recurse(a: &mut Vec<char>, b: &mut Vec<char>, carry: u32, output: &mut String) {
        let x = a.pop();
        let y = b.pop();
        if x.is_none() && y.is_none() {
            if carry > 0 {
                output.push('1');
            }
            return;
        }

        let x = x.unwrap_or('0');
        let y = y.unwrap_or('0');
        let x: u32 = x.to_digit(10).unwrap();
        let y: u32 = y.to_digit(10).unwrap();

        let mut sum = carry + x + y;
        let mut next_carry = 0;
        if sum > 1 {
            next_carry = 1;
            sum %= 2;
        }

        let c = match sum {
            0 => '0',
            1 => '1',
            _ => panic!("unexpected digit!"),
        };

        output.push(c);

        Solution::recurse(a, b, next_carry, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = "11".to_owned();
        let b = "1".to_owned();

        let output = Solution::add_binary(a, b);

        assert_eq!("100".to_owned(), output);
    }

    #[test]
    fn example2() {
        let a = "1010".to_owned();
        let b = "1011".to_owned();

        let output = Solution::add_binary(a, b);

        assert_eq!("10101".to_owned(), output);
    }

    #[test]
    fn wrong_answer1() {
        let a = "1111".to_owned();
        let b = "1111".to_owned();

        let output = Solution::add_binary(a, b);

        assert_eq!("11110".to_owned(), output);
    }
}
