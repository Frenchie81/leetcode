// problem 13
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;

        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let c = chars[i];
            let next_c = chars.get(i + 1);

            let amount = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("invalid roman numeral!"),
            };

            let should_subtract = if let Some(next_c) = next_c {
                let next_c = *next_c;
                match c {
                    'I' => next_c == 'V' || next_c == 'X',
                    'X' => next_c == 'L' || next_c == 'C',
                    'C' => next_c == 'D' || next_c == 'M',
                    _ => false,
                }
            } else {
                false
            };

            if should_subtract {
                sum -= amount;
            } else {
                sum += amount;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "III".to_string();

        let output = Solution::roman_to_int(input);

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let input = "LVIII".to_string();

        let output = Solution::roman_to_int(input);

        assert_eq!(58, output);
    }

    #[test]
    fn example3() {
        let input = "MCMXCIV".to_string();

        let output = Solution::roman_to_int(input);

        assert_eq!(1994, output);
    }
}
