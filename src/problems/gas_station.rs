// problem 134
struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.is_empty() {
            return -1;
        }

        if gas.len() == 1 {
            if gas[0] >= cost[0] {
                return 0;
            } else {
                return -1;
            }
        }

        let mut sum = 0;
        let mut run = 0;
        let mut output = -1;

        for (i, gas) in gas.iter().enumerate() {
            let cost = cost[i];
            let diff = gas - cost;
            run += diff;

            if output == -1 && run >= 0 {
                output = i as i32;
            }

            if run < 0 {
                run = 0;
                output = -1;
            }

            sum += diff;
        }

        if sum >= 0 {
            output
        } else {
            -1
        }
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

    #[test]
    fn wrong_answer1() {
        let gas = vec![0, 0, 0, 0, 0, 0, 2];
        let cost = vec![0, 0, 0, 0, 0, 1, 0];

        let output = Solution::can_complete_circuit(gas, cost);

        assert_eq!(6, output);
    }

    #[test]
    fn wrong_answer2() {
        let gas = vec![2];
        let cost = vec![2];

        let output = Solution::can_complete_circuit(gas, cost);

        assert_eq!(0, output);
    }

    #[test]
    fn wrong_answer3() {
        let gas = vec![5, 1, 2, 3, 4];
        let cost = vec![4, 4, 1, 5, 1];
        //                      1,-3, 1,-2, 3
        let output = Solution::can_complete_circuit(gas, cost);

        assert_eq!(4, output);
    }
}
