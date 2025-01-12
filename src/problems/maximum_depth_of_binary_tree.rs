use std::cell::RefCell;
use std::rc::Rc;

// problem 104
struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeWrap>,
    pub right: Option<TreeNodeWrap>,
}

type TreeNodeWrap = Rc<RefCell<TreeNode>>;

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let borrowed = root.borrow();

        let left = 1 + Self::max_depth(borrowed.left.clone());
        let right = 1 + Self::max_depth(borrowed.right.clone());

        if left > right {
            left
        } else {
            right
        }
    }
}

fn create_node(val: i32) -> TreeNodeWrap {
    TreeNodeWrap::new(RefCell::new(TreeNode::new(val)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let temp = create_node(3);
        let mut root = temp.borrow_mut();
        root.left = Some(create_node(9));

        let right = create_node(20);
        let mut mutable_right = right.borrow_mut();
        mutable_right.left = Some(create_node(15));
        mutable_right.right = Some(create_node(7));
        drop(mutable_right);

        root.right = Some(right);

        drop(root);

        let output = Solution::max_depth(Some(temp));

        assert_eq!(3, output);
    }

    #[test]
    fn example2() {
        let temp = create_node(1);
        let mut root = temp.borrow_mut();
        root.right = Some(create_node(2));

        drop(root);

        let output = Solution::max_depth(Some(temp));

        assert_eq!(2, output);
    }
}
