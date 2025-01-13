use std::cell::RefCell;
use std::rc::Rc;

// problem 100
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if (p.is_none() && q.is_some()) || (q.is_none() && p.is_some()) {
            return false;
        }

        if p.is_none() && q.is_none() {
            return true;
        }

        let p = p.unwrap();
        let q = q.unwrap();
        let p = p.borrow();
        let q = q.borrow();

        if p.val == q.val {
            let left_result = Self::is_same_tree(p.left.clone(), q.left.clone());
            if !left_result {
                return false;
            }

            let right_result = Self::is_same_tree(p.right.clone(), q.right.clone());
            if !right_result {
                return false;
            }

            true
        } else {
            false
        }
    }
}

fn wrap_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(n)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let p = TreeNode {
            val: 1,
            left: wrap_node(TreeNode::new(2)),
            right: wrap_node(TreeNode::new(3)),
        };

        let q = TreeNode {
            val: 1,
            left: wrap_node(TreeNode::new(2)),
            right: wrap_node(TreeNode::new(3)),
        };

        let output = Solution::is_same_tree(wrap_node(p), wrap_node(q));

        assert!(output);
    }

    #[test]
    fn example2() {
        let p = TreeNode {
            val: 1,
            left: wrap_node(TreeNode::new(2)),
            right: None,
        };

        let q = TreeNode {
            val: 1,
            left: None,
            right: wrap_node(TreeNode::new(2)),
        };

        let output = Solution::is_same_tree(wrap_node(p), wrap_node(q));

        assert!(!output);
    }

    #[test]
    fn example3() {
        let p = TreeNode {
            val: 1,
            left: wrap_node(TreeNode::new(2)),
            right: wrap_node(TreeNode::new(1)),
        };

        let q = TreeNode {
            val: 1,
            left: wrap_node(TreeNode::new(1)),
            right: wrap_node(TreeNode::new(2)),
        };

        let output = Solution::is_same_tree(wrap_node(p), wrap_node(q));

        assert!(!output);
    }
}
