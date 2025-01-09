// problem 228
struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }

        let mut ranges = Vec::new();
        let mut start_range = nums[0];
        let mut end_range = nums[0];
        for i in 1..nums.len() {
            let prev = nums[i - 1];
            let cur = nums[i];

            if cur - prev != 1 {
                if start_range == end_range {
                    ranges.push(start_range.to_string());
                } else {
                    ranges.push(format!("{start_range}->{end_range}"));
                }

                start_range = cur;
                end_range = cur;
            } else {
                end_range = cur;
            }
        }

        // clean up the last range after looping
        if start_range == end_range {
            ranges.push(start_range.to_string());
        } else {
            ranges.push(format!("{start_range}->{end_range}"));
        }

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![0, 1, 2, 4, 5, 7];

        let output = Solution::summary_ranges(nums);

        assert_eq!(
            vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()],
            output
        );
    }

    #[test]
    fn example2() {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];

        let output = Solution::summary_ranges(nums);

        assert_eq!(
            vec![
                "0".to_string(),
                "2->4".to_string(),
                "6".to_string(),
                "8->9".to_string()
            ],
            output
        );
    }
}
