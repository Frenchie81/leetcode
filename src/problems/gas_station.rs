// problem 134
struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for (i, val) in gas.iter().enumerate() {
            let mut starting_gas = *val;
            let result = Self::recurse(&gas, &cost, i, i + 1, &mut starting_gas);

            if result > -1 {
                return result;
            }
        }

        -1
    }

    fn recurse(
        gas: &[i32],
        cost: &[i32],
        start_pos: usize,
        cur_pos: usize,
        cur_gas: &mut i32,
    ) -> i32 {
        let mut cur_pos = cur_pos;
        if cur_pos >= gas.len() {
            cur_pos -= gas.len();
        }

        *cur_gas -= cost[cur_pos];
        if *cur_gas < 0 {
            return -1;
        }

        if start_pos == cur_pos {
            return start_pos as i32;
        }

        *cur_gas += gas[cur_pos];

        Self::recurse(gas, cost, start_pos, cur_pos + 1, cur_gas)
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
