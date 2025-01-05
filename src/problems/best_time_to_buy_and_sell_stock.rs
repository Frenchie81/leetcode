// problem 121
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest = *prices.first().unwrap();
        let mut highest = lowest;
        let mut diffs = vec![];

        (1..prices.len()).for_each(|i| {
            let price = prices[i];
            if price < lowest {
                let diff = highest - lowest;
                if diff > 0 {
                    diffs.push(diff);
                }
                lowest = price;
                highest = price;
            } else if price > highest {
                highest = price;
            }
        });

        let diff = highest - lowest;
        if diff > 0 {
            diffs.push(diff);
        }

        *diffs.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let prices = vec![7, 1, 5, 3, 6, 4];

        let output = Solution::max_profit(prices);

        assert_eq!(5, output);
    }

    #[test]
    fn example2() {
        let prices = vec![7, 6, 4, 3, 1];

        let output = Solution::max_profit(prices);

        assert_eq!(0, output);
    }

    #[test]
    fn wrong_answer1() {
        let prices = vec![2, 4, 1];

        let output = Solution::max_profit(prices);

        assert_eq!(2, output);
    }
}
