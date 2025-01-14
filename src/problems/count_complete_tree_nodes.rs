// problem 222
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let root = root.borrow();
        let left_max_depth = Self::get_max_depth(root.left.clone(), true);
        let right_max_depth = Self::get_max_depth(root.right.clone(), false);

        if left_max_depth == right_max_depth {
            // perfect
            2_i32.pow((left_max_depth as u32) + 1) - 1
        } else {
            let left_total = Self::count_nodes(root.left.clone());
            let right_total = Self::count_nodes(root.right.clone());
            left_total + right_total + 1
        }
    }

    fn get_max_depth(root: Option<Rc<RefCell<TreeNode>>>, left: bool) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.unwrap();
        let node = node.borrow();
        if left {
            1 + Self::get_max_depth(node.left.clone(), left)
        } else {
            1 + Self::get_max_depth(node.right.clone(), left)
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
            val: 1,
            left: Solution::wrap_node(TreeNode {
                val: 2,
                left: Solution::wrap_node(TreeNode::new(4)),
                right: Solution::wrap_node(TreeNode::new(5)),
            }),
            right: Solution::wrap_node(TreeNode {
                val: 3,
                left: Solution::wrap_node(TreeNode::new(6)),
                right: None,
            }),
        });

        let output = Solution::count_nodes(input);

        assert_eq!(6, output);
    }

    #[test]
    fn example2() {
        let input = None;

        let output = Solution::count_nodes(input);

        assert_eq!(0, output);
    }

    #[test]
    fn example3() {
        let input = Solution::wrap_node(TreeNode::new(1));

        let output = Solution::count_nodes(input);

        assert_eq!(1, output);
    }

    #[test]
    fn wrong_answer1() {
        let input = Solution::wrap_node(TreeNode {
            val: 1,
            left: Solution::wrap_node(TreeNode::new(2)),
            right: None,
        });

        let output = Solution::count_nodes(input);

        assert_eq!(2, output);
    }

    #[test]
    fn wrong_answer2() {
        let input = Solution::wrap_node(TreeNode {
            val: 1,
            left: Solution::wrap_node(TreeNode {
                val: 2,
                left: Solution::wrap_node(TreeNode::new(4)),
                right: None,
            }),
            right: Solution::wrap_node(TreeNode {
                val: 3,
                left: None,
                right: None,
            }),
        });

        let output = Solution::count_nodes(input);

        assert_eq!(4, output);
    }
}
