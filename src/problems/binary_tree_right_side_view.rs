// problem 199
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let root = root.unwrap();
        let mut queue = VecDeque::new();
        let mut result = vec![];

        queue.push_back((0, root));

        while let Some((depth, item)) = queue.pop_front() {
            let item = item.borrow();

            if let Some(existing) = result.get_mut(depth) {
                *existing = item.val;
            } else {
                result.push(item.val);
            }

            if let Some(ref left) = item.left {
                queue.push_back((depth + 1, Rc::clone(left)));
            }

            if let Some(ref right) = item.right {
                queue.push_back((depth + 1, Rc::clone(right)));
            }
        }

        result
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
            val: 1,
            left: wrap(TreeNode {
                val: 2,
                left: None,
                right: wrap(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }),
            }),
            right: wrap(TreeNode {
                val: 3,
                left: None,
                right: wrap(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }),
            }),
        });

        let output = Solution::right_side_view(root);

        assert_eq!(vec![1, 3, 4], output);
    }

    #[test]
    fn example2() {
        let root = wrap(TreeNode {
            val: 1,
            left: wrap(TreeNode {
                val: 2,
                left: wrap(TreeNode {
                    val: 4,
                    left: wrap(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }),
                    right: None,
                }),
                right: None,
            }),
            right: wrap(TreeNode {
                val: 3,
                left: None,
                right: None,
            }),
        });

        let output = Solution::right_side_view(root);

        assert_eq!(vec![1, 3, 4, 5], output);
    }

    #[test]
    fn example3() {
        let root = wrap(TreeNode {
            val: 1,
            left: None,
            right: wrap(TreeNode {
                val: 3,
                left: None,
                right: None,
            }),
        });

        let output = Solution::right_side_view(root);

        assert_eq!(vec![1, 3], output);
    }

    #[test]
    fn example4() {
        let root = None;

        let output = Solution::right_side_view(root);

        assert!(output.is_empty());
    }
}
