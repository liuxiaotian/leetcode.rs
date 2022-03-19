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

/// Solution for [606. Construct String from Binary Tree](https://leetcode-cn.com/problems/construct-string-from-binary-tree/)
pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(t) = t {
            match (t.borrow().left.clone(), t.borrow().right.clone()) {
                (None, None) => t.borrow().val.to_string(),
                (_, None) => format!(
                    "{}({})",
                    t.borrow().val.to_string(),
                    Solution::tree2str(t.borrow().left.clone())
                ),
                (_, _) => format!(
                    "{}({})({})",
                    t.borrow().val.to_string(),
                    Solution::tree2str(t.borrow().left.clone()),
                    Solution::tree2str(t.borrow().right.clone())
                ),
            }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec2tree(v: Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if i >= v.len() {
            None
        } else {
            match v[i] {
                Some(x) => {
                    let mut node = TreeNode::new(x);
                    node.left = vec2tree(v.clone(), i * 2 + 1);
                    node.right = vec2tree(v.clone(), i * 2 + 2);
                    Some(Rc::new(RefCell::new(node)))
                }
                None => None,
            }
        }
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::tree2str(vec2tree(vec![Some(1), Some(2), Some(3), Some(4)], 0)),
            "1(2(4))(3)"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::tree2str(vec2tree(vec![Some(1), Some(2), Some(3), None, Some(4)], 0)),
            "1(2()(4))(3)"
        );
    }
}
