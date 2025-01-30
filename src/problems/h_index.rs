// problem 274
struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a, b| b.cmp(a));

        let mut h = 0;
        for (i, c) in citations.iter().enumerate() {
            let i = i + 1;
            if i as i32 <= *c {
                h = i;
            } else {
                break;
            }
        }
        h as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let citations = vec![3, 0, 6, 1, 5];

        let output = Solution::h_index(citations);

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let citations = vec![1, 3, 1];

        let output = Solution::h_index(citations);

        assert_eq!(1, output);
    }
}
