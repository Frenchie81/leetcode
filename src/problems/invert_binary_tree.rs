// problem 226
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;

        let root = root.unwrap();
        let root = root.borrow();
        wrap_node(TreeNode {
            val: root.val,
            left: Self::invert_tree(root.right.clone()),
            right: Self::invert_tree(root.left.clone()),
        })
    }
}

fn wrap_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(n)))
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let left_result = is_same_tree(p.left.clone(), q.left.clone());
            if !left_result {
                return false;
            }

            let right_result = is_same_tree(p.right.clone(), q.right.clone());
            if !right_result {
                return false;
            }

            true
        } else {
            false
        }
    }

    #[test]
    fn example1() {
        let input = wrap_node(TreeNode {
            val: 4,
            left: wrap_node(TreeNode {
                val: 2,
                left: wrap_node(TreeNode::new(1)),
                right: wrap_node(TreeNode::new(3)),
            }),
            right: wrap_node(TreeNode {
                val: 7,
                left: wrap_node(TreeNode::new(6)),
                right: wrap_node(TreeNode::new(9)),
            }),
        });

        let output = Solution::invert_tree(input);

        let expected = wrap_node(TreeNode {
            val: 4,
            left: wrap_node(TreeNode {
                val: 7,
                left: wrap_node(TreeNode::new(9)),
                right: wrap_node(TreeNode::new(6)),
            }),
            right: wrap_node(TreeNode {
                val: 2,
                left: wrap_node(TreeNode::new(3)),
                right: wrap_node(TreeNode::new(1)),
            }),
        });

        assert!(is_same_tree(expected, output));
    }

    #[test]
    fn example2() {
        let input = wrap_node(TreeNode {
            val: 4,
            left: wrap_node(TreeNode::new(1)),
            right: wrap_node(TreeNode::new(3)),
        });

        let output = Solution::invert_tree(input);

        let expected = wrap_node(TreeNode {
            val: 4,
            left: wrap_node(TreeNode::new(3)),
            right: wrap_node(TreeNode::new(1)),
        });

        assert!(is_same_tree(expected, output));
    }

    #[test]
    fn example3() {
        let output = Solution::invert_tree(None);

        let expected = None;

        assert!(is_same_tree(expected, output));
    }
}
