// problem 230
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut ordered = vec![];
        Solution::recurse(root, &mut ordered, k as usize);

        let index = (k as usize) - 1;
        if let Some(val) = ordered.get(index) {
            *val
        } else {
            0
        }
    }

    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, ordered: &mut Vec<i32>, max_length: usize) {
        if node.is_none() {
            return;
        }

        let node = node.unwrap();
        let node = node.borrow();

        Solution::recurse(node.left.clone(), ordered, max_length);
        ordered.push(node.val);
        if ordered.len() == max_length {
            return;
        }
        Solution::recurse(node.right.clone(), ordered, max_length);
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
        let root = wrap(TreeNode {
            val: 3,
            left: wrap(TreeNode {
                val: 1,
                left: None,
                right: wrap(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }),
            }),
            right: wrap(TreeNode {
                val: 4,
                left: None,
                right: None,
            }),
        });
        let k = 1;

        let output = Solution::kth_smallest(root, k);

        assert_eq!(1, output);
    }

    #[test]
    fn example2() {
        let root = wrap(TreeNode {
            val: 5,
            left: wrap(TreeNode {
                val: 3,
                left: wrap(TreeNode {
                    val: 2,
                    left: wrap(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }),
                    right: None,
                }),
                right: wrap(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }),
            }),
            right: wrap(TreeNode {
                val: 6,
                left: None,
                right: None,
            }),
        });
        let k = 3;

        let output = Solution::kth_smallest(root, k);

        assert_eq!(3, output);
    }
}
