// problem 105
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(&preorder, &mut 0, &inorder)
    }

    fn recurse(
        preorder: &[i32],
        index: &mut usize,
        inorder: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if *index >= preorder.len() {
            return None;
        }

        let val = preorder[*index];
        let split_pos = inorder.iter().position(|v| *v == val).unwrap();

        let left = &inorder[0..split_pos];
        let right = &inorder[split_pos + 1..inorder.len()];

        let mut node = TreeNode::new(val);
        *index += 1;

        if !left.is_empty() {
            node.left = Self::recurse(preorder, index, left);
        }
        if !right.is_empty() {
            node.right = Self::recurse(preorder, index, right);
        }

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn example1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let output = Solution::build_tree(preorder, inorder);

        let expected = wrap(TreeNode {
            val: 3,
            left: wrap(TreeNode {
                val: 9,
                left: None,
                right: None,
            }),
            right: wrap(TreeNode {
                val: 20,
                left: wrap(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }),
                right: wrap(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }),
            }),
        });

        assert_eq!(expected, output);
    }

    #[test]
    fn example2() {
        let preorder = vec![-1];
        let inorder = vec![-1];

        let output = Solution::build_tree(preorder, inorder);

        let expected = wrap(TreeNode {
            val: -1,
            left: None,
            right: None,
        });

        assert_eq!(expected, output);
    }
}
