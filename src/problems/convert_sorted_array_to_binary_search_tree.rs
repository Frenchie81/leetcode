// problem 108
struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::recurse(&nums)
    }

    fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let middle = nums.len() / 2;
        let node = TreeNode {
            val: nums[middle],
            left: Solution::recurse(&nums[0..middle]),
            right: Solution::recurse(&nums[middle + 1..nums.len()]),
        };

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }

    #[test]
    fn example1() {
        let input = vec![-10, -3, 0, 5, 9];

        let output = Solution::sorted_array_to_bst(input);

        let expected = wrap_node(TreeNode {
            val: 0,
            left: wrap_node(TreeNode {
                val: -3,
                left: wrap_node(TreeNode {
                    val: -10,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
            right: wrap_node(TreeNode {
                val: 9,
                left: wrap_node(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }),
                right: None,
            }),
        });

        assert_eq!(expected, output);
    }

    #[test]
    fn example2() {
        let input = vec![1, 3];

        let output = Solution::sorted_array_to_bst(input);

        let expected = wrap_node(TreeNode {
            val: 3,
            left: wrap_node(TreeNode {
                val: 1,
                left: None,
                right: None,
            }),
            right: None,
        });

        assert_eq!(expected, output);
    }
}
