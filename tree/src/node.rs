// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

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

    // 前序遍历
    pub fn preorder(&self) -> Vec<i32> {
        fn helper(node: &TreeNode, accumulator: &mut Vec<i32>) {
            // Proccess val here for preorder traversal. [Node, L, R]
            accumulator.push(node.val);
            // Only care about one arm of the match, so use `if let`
            if let Some(ref left) = node.left {
                helper(&left.borrow(), accumulator);
            }
            // Hint: Proccess val here if we were doing inorder traversal. [L, Node, R]
            if let Some(right) = &node.right {
                helper(&right.borrow(), accumulator);
            }
            // Hint: Proccess val here if we were doing postorder traversal. [L, R, Node]
        }
        let mut accumulator = vec![];
        helper(self, &mut accumulator);
        accumulator
    }

    // 中序遍历
    pub fn inorder(&self) -> Vec<i32> {
        fn helper(node: &TreeNode, accumulator: &mut Vec<i32>) {
            if let Some(ref left) = node.left {
                helper(&left.borrow(), accumulator)
            }
            accumulator.push(node.val);
            if let Some(right) = &node.right {
                helper(&right.borrow(), accumulator)
            }
        }

        let mut accumulator = vec![];
        helper(self, &mut accumulator);
        return accumulator;
    }

    // 后序遍历
    pub fn postorder(&self) -> Vec<i32> {
        fn helper(node: &TreeNode, accumulator: &mut Vec<i32>) {
            if let Some(ref left) = node.left {
                helper(&left.borrow(), accumulator)
            }
            if let Some(right) = &node.right {
                helper(&right.borrow(), accumulator)
            }
            accumulator.push(node.val);
        }

        let mut accumulator = vec![];
        helper(self, &mut accumulator);
        return accumulator;
    }

    // 水平遍历
    // pub fn level_order(&self) -> Vec<i32> {
    //     fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //         let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    //         let mut ans: Vec<Vec<i32>> = Vec::new();

    //         if let Some(node) = root {
    //             queue.push_back(node);
    //         } else {
    //             return ans;
    //         }

    //         while !queue.is_empty() {
    //             ans.push(queue.iter().map(|node| node.borrow().val).collect());
    //             for _ in 0..queue.len() {
    //                 if let Some(node) = queue.pop_front() {
    //                     if let Some(left) = node.borrow().left.clone() {
    //                         queue.push_back(Rc::clone(&left));
    //                     }
    //                     if let Some(right) = node.borrow().right.clone() {
    //                         queue.push_back(Rc::clone(&right));
    //                     }
    //                 }
    //             }
    //         }
    //         ans
    //     }
    //     return helper(self);
    // }
}
