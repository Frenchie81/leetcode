// problem 122
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut last_price = prices[0];

        (1..prices.len()).for_each(|i| {
            let cur_price = prices[i];
            if cur_price > last_price {
                profit += cur_price - last_price;
            }
            last_price = cur_price;
        });

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![7, 1, 5, 3, 6, 4];

        let output = Solution::max_profit(input);

        assert_eq!(7, output);
    }

    #[test]
    fn example2() {
        let input = vec![1, 2, 3, 4, 5];

        let output = Solution::max_profit(input);

        assert_eq!(4, output);
    }

    #[test]
    fn example3() {
        let input = vec![7, 6, 4, 3, 1];

        let output = Solution::max_profit(input);

        assert_eq!(0, output);
    }
}
