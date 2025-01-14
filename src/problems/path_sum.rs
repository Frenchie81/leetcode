// problem 112
use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::recurse(root, 0, target_sum)
    }

    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, current_sum: i32, target_sum: i32) -> bool {
        if node.is_none() {
            return false;
        }

        let node = node.unwrap();
        let node = node.borrow();

        let new_sum = current_sum + node.val;
        if target_sum == new_sum && node.left.is_none() && node.right.is_none() {
            return true;
        }

        if Self::recurse(node.left.clone(), new_sum, target_sum) {
            true
        } else {
            Self::recurse(node.right.clone(), new_sum, target_sum)
        }
    }

    fn wrap_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Solution::wrap_node(TreeNode {
            val: 5,
            left: Solution::wrap_node(TreeNode {
                val: 4,
                left: Solution::wrap_node(TreeNode {
                    val: 11,
                    left: Solution::wrap_node(TreeNode::new(7)),
                    right: Solution::wrap_node(TreeNode::new(2)),
                }),
                right: None,
            }),
            right: Solution::wrap_node(TreeNode {
                val: 8,
                left: Solution::wrap_node(TreeNode::new(13)),
                right: Solution::wrap_node(TreeNode {
                    val: 4,
                    left: None,
                    right: Solution::wrap_node(TreeNode::new(1)),
                }),
            }),
        });

        let output = Solution::has_path_sum(input, 22);

        assert!(output);
    }

    #[test]
    fn example2() {
        let input = Solution::wrap_node(TreeNode {
            val: 1,
            left: Solution::wrap_node(TreeNode::new(2)),
            right: Solution::wrap_node(TreeNode::new(3)),
        });

        let output = Solution::has_path_sum(input, 5);

        assert!(!output);
    }

    #[test]
    fn example3() {
        let output = Solution::has_path_sum(None, 0);

        assert!(!output);
    }

    #[test]
    fn wrong_answer1() {
        let input = Solution::wrap_node(TreeNode {
            val: 1,
            left: Solution::wrap_node(TreeNode::new(2)),
            right: None,
        });

        let output = Solution::has_path_sum(input, 1);

        assert!(!output);
    }
}
