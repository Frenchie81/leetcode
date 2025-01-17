// problem 530
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return i32::MAX;
        }

        let mut diff = BstMinDifference {
            min: i32::MAX,
            prev: None,
        };

        diff.get_min_diff(root);
        diff.min
    }
}

struct BstMinDifference {
    min: i32,
    prev: Option<i32>,
}

impl BstMinDifference {
    fn get_min_diff(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return self.min;
        }

        let root = root.unwrap();
        let root = root.borrow();

        self.get_min_diff(root.left.clone());

        if let Some(p) = self.prev {
            let diff = (p - root.val).abs();
            if diff < self.min {
                self.min = diff;
            }
        }

        self.prev = Some(root.val);

        self.get_min_diff(root.right.clone());

        self.min
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
        let input = wrap_node(TreeNode {
            val: 4,
            left: wrap_node(TreeNode {
                val: 2,
                left: wrap_node(TreeNode::new(1)),
                right: wrap_node(TreeNode::new(3)),
            }),
            right: wrap_node(TreeNode::new(6)),
        });

        let output = Solution::get_minimum_difference(input);

        assert_eq!(1, output);
    }

    #[test]
    fn example2() {
        let input = wrap_node(TreeNode {
            val: 1,
            left: wrap_node(TreeNode {
                val: 0,
                left: None,
                right: None,
            }),
            right: wrap_node(TreeNode {
                val: 48,
                left: wrap_node(TreeNode {
                    val: 12,
                    left: None,
                    right: None,
                }),
                right: wrap_node(TreeNode {
                    val: 49,
                    left: None,
                    right: None,
                }),
            }),
        });

        let output = Solution::get_minimum_difference(input);

        assert_eq!(1, output);
    }

    #[test]
    fn wrong_answer1() {
        let input = wrap_node(TreeNode {
            val: 236,
            left: wrap_node(TreeNode {
                val: 104,
                left: None,
                right: wrap_node(TreeNode {
                    val: 227,
                    left: None,
                    right: None,
                }),
            }),
            right: wrap_node(TreeNode {
                val: 701,
                left: None,
                right: wrap_node(TreeNode {
                    val: 49,
                    left: None,
                    right: wrap_node(TreeNode {
                        val: 911,
                        left: None,
                        right: None,
                    }),
                }),
            }),
        });

        let output = Solution::get_minimum_difference(input);

        assert_eq!(9, output);
    }
}
