// problem 2
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::recurse(l1, l2, 0)
    }

    fn recurse(
        node1: Option<Box<ListNode>>,
        node2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if node1.is_none() && node2.is_none() {
            if carry > 0 {
                return Some(Box::new(ListNode::new(carry)));
            } else {
                return None;
            }
        }

        let mut val1 = 0;
        let mut val2 = 0;

        let mut next_node1 = None;
        let mut next_node2 = None;

        if let Some(v) = node1 {
            val1 = v.val;
            next_node1 = v.next;
        }

        if let Some(v) = node2 {
            val2 = v.val;
            next_node2 = v.next;
        }

        let mut sum = val1 + val2 + carry;
        let carry = if sum > 9 {
            sum -= 10;
            1
        } else {
            0
        };

        Some(Box::new(ListNode {
            val: sum,
            next: Solution::recurse(next_node1, next_node2, carry),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wrap(node: ListNode) -> Option<Box<ListNode>> {
        Some(Box::new(node))
    }

    fn to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = node;
        while let Some(ref n) = current {
            vec.push(n.val);
            current = n.next.clone()
        }
        vec
    }

    fn from_vec(values: &[i32], index: usize) -> Option<Box<ListNode>> {
        if let Some(value) = values.get(index) {
            wrap(ListNode {
                val: *value,
                next: from_vec(values, index + 1),
            })
        } else {
            None
        }
    }

    #[test]
    fn example1() {
        let l1 = from_vec(&[2, 4, 3], 0);
        let l2 = from_vec(&[5, 6, 4], 0);

        let output = Solution::add_two_numbers(l1, l2);

        assert_eq!(vec![7, 0, 8], to_vec(output));
    }

    #[test]
    fn example2() {
        let l1 = wrap(ListNode { val: 0, next: None });
        let l2 = wrap(ListNode { val: 0, next: None });

        let output = Solution::add_two_numbers(l1, l2);

        assert_eq!(vec![0], to_vec(output));
    }

    #[test]
    fn example3() {
        let l1 = from_vec(&[9, 9, 9, 9, 9, 9, 9], 0);
        let l2 = from_vec(&[9, 9, 9, 9], 0);

        let output = Solution::add_two_numbers(l1, l2);

        assert_eq!(vec![8, 9, 9, 9, 0, 0, 0, 1], to_vec(output));
    }
}
