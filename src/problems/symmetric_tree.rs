// problem 101
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();
        let root = root.borrow();

        Self::recurse(root.left.clone(), root.right.clone())
    }

    fn recurse(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() && right.is_some() {
            return false;
        }

        if left.is_some() && right.is_none() {
            return false;
        }

        let left = left.unwrap();
        let right = right.unwrap();
        let left = left.borrow();
        let right = right.borrow();

        if left.val != right.val {
            return false;
        }

        let next_val = Self::recurse(left.left.clone(), right.right.clone());
        if !next_val {
            return false;
        }

        Self::recurse(left.right.clone(), right.left.clone())
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
            val: 1,
            left: Solution::wrap_node(TreeNode {
                val: 2,
                left: Solution::wrap_node(TreeNode::new(3)),
                right: Solution::wrap_node(TreeNode::new(4)),
            }),
            right: Solution::wrap_node(TreeNode {
                val: 2,
                left: Solution::wrap_node(TreeNode::new(4)),
                right: Solution::wrap_node(TreeNode::new(3)),
            }),
        });

        let output = Solution::is_symmetric(input);

        assert!(output);
    }

    #[test]
    fn example2() {
        let input = Solution::wrap_node(TreeNode {
            val: 1,
            left: Solution::wrap_node(TreeNode {
                val: 2,
                left: None,
                right: Solution::wrap_node(TreeNode::new(3)),
            }),
            right: Solution::wrap_node(TreeNode {
                val: 2,
                left: None,
                right: Solution::wrap_node(TreeNode::new(3)),
            }),
        });

        let output = Solution::is_symmetric(input);

        assert!(!output);
    }
}
