// problem 21
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            None
        } else if list1.is_some() && list2.is_none() {
            list1
        } else if list1.is_none() && list2.is_some() {
            list2
        } else {
            let node1 = list1.unwrap();
            let node2 = list2.unwrap();
            if node1.val <= node2.val {
                let mut new_root = node1.clone();
                new_root.next = Self::merge_two_lists(new_root.next, Some(node2));
                return Some(new_root);
            } else {
                let mut new_root = node2.clone();
                new_root.next = Self::merge_two_lists(Some(node1), new_root.next);
                return Some(new_root);
            }
        }
    }
}

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

fn create_list(nums: &[i32], cur_index: usize) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }

    if let Some(num) = nums.get(cur_index) {
        let mut node = Box::new(ListNode::new(*num));
        node.next = create_list(nums, cur_index + 1);
        Some(node)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let list1 = create_list(&[1, 2, 4], 0);
        let list2 = create_list(&[1, 3, 4], 0);

        let output = Solution::merge_two_lists(list1, list2);

        assert!(output.is_some());
        let output = output.unwrap();
        let expected = create_list(&[1, 1, 2, 3, 4, 4], 0).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn example2() {
        let list1 = create_list(&[], 0);
        let list2 = create_list(&[], 0);

        let output = Solution::merge_two_lists(list1, list2);

        assert!(output.is_none());
    }

    #[test]
    fn example3() {
        let list1 = create_list(&[], 0);
        let list2 = create_list(&[0], 0);

        let output = Solution::merge_two_lists(list1, list2);

        assert!(output.is_some());
        let output = output.unwrap();
        let expected = create_list(&[0], 0).unwrap();
        assert_eq!(expected, output);
    }
}
