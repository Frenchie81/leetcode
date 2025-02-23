// problem 134
struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];

        let output = Solution::can_complete_circuit(gas, cost);

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];

        let output = Solution::can_complete_circuit(gas, cost);

        assert_eq!(-1, output);
    }
}
