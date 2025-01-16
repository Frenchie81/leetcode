// problem 637
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return Vec::new();
        }

        let root = root.unwrap();
        let mut results: Vec<Vec<f64>> = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, root));

        while let Some((depth, item)) = queue.pop_front() {
            let item = item.borrow();
            if let Some(existing) = results.get_mut(depth) {
                existing.push(item.val as f64);
            } else {
                results.push(vec![item.val as f64]);
            }

            if let Some(ref left) = item.left {
                queue.push_back((depth + 1, Rc::clone(left)));
            }

            if let Some(ref right) = item.right {
                queue.push_back((depth + 1, Rc::clone(right)));
            }
        }

        results
            .iter()
            .map(|f| f.iter().copied().sum::<f64>() / f.len() as f64)
            .collect()
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
        let input = wrap_node(TreeNode {
            val: 3,
            left: wrap_node(TreeNode::new(9)),
            right: wrap_node(TreeNode {
                val: 20,
                left: wrap_node(TreeNode::new(15)),
                right: wrap_node(TreeNode::new(7)),
            }),
        });

        let output = Solution::average_of_levels(input);

        assert_eq!(vec![3.0, 14.5, 11.0], output);
    }

    #[test]
    fn example2() {
        let input = wrap_node(TreeNode {
            val: 3,
            left: wrap_node(TreeNode {
                val: 9,
                left: wrap_node(TreeNode::new(15)),
                right: wrap_node(TreeNode::new(7)),
            }),
            right: wrap_node(TreeNode::new(20)),
        });

        let output = Solution::average_of_levels(input);

        assert_eq!(vec![3.0, 14.5, 11.0], output);
    }
}
