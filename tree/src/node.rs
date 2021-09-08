// Definition for a binary tree node.
// use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::vec::Vec;
use std::{cell::RefCell, fmt::Result};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

// impl Display for TreeNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
//         let preorder_list = self.preorder();
//         let inorder_list = self.inorder();
//         let postorder_list = self.postorder();
//         write!(
//             f,
//             "preorder:({:?}), inorder({:?}), postorder({:?})",
//             preorder_list, inorder_list, postorder_list
//         )
//     }
// }

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }

    // 以前序遍历和中序遍历生成TreeNode
    pub fn new_with_pre_and_in(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() != inorder.len() || preorder.len() == 0 || inorder.len() == 0 {
            return None;
        }

        let mut root = Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: None,
            right: None,
            next: None,
        }));

        let mut in_pos = 0usize;
        while in_pos < inorder.len() {
            if preorder[0] == inorder[in_pos] {
                // inorder中inPos左边是左子树
                root.borrow_mut().left = TreeNode::new_with_pre_and_in(
                    &preorder[1..in_pos + 1].to_vec(),
                    &inorder[..in_pos].to_vec(),
                );
                // inorder中inPos右边是右子树
                root.borrow_mut().right = TreeNode::new_with_pre_and_in(
                    &preorder[in_pos + 1..].to_vec(),
                    &inorder[in_pos + 1..].to_vec(),
                );
                // 找到了就退出此次循环
                break;
            }
            in_pos += 1;
        }

        return Some(root);
    }

    // 根据中序和后序遍历生成TreeNode
    pub fn new_with_in_and_post(
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() != postorder.len() || inorder.len() == 0 || postorder.len() == 0 {
            return None;
        }

        let mut root = Rc::new(RefCell::new(TreeNode {
            val: postorder[postorder.len() - 1],
            left: None,
            right: None,
            next: None,
        }));
        let mut in_pos = 0usize;
        while in_pos < inorder.len() {
            if inorder[in_pos] == postorder[postorder.len() - 1] {
                root.borrow_mut().left = TreeNode::new_with_in_and_post(
                    &inorder[..in_pos].to_vec(),
                    &postorder[..in_pos].to_vec(),
                );
                root.borrow_mut().right = TreeNode::new_with_in_and_post(
                    &inorder[in_pos + 1..].to_vec(),
                    &postorder[in_pos..postorder.len() - 1].to_vec(),
                );
                break;
            }
            in_pos += 1;
        }
        return Some(root);
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
    pub fn level_order(&self) -> Vec<Vec<i32>> {
        fn helper(node: &TreeNode, result: &mut Vec<Vec<i32>>, depth: usize) {
            if depth == result.len() {
                result.push(Vec::<i32>::new())
            }
            result[depth].push(node.val);
            if let Some(ref left) = node.left {
                helper(&left.borrow(), result, depth + 1)
            }
            if let Some(ref right) = node.right {
                helper(&right.borrow(), result, depth + 1)
            }
        }

        let mut result = Vec::<Vec<i32>>::new();
        helper(self, &mut result, 0);
        return result;
    }

    // 树最大深度
    pub fn max_depth(&self) -> i32 {
        fn helper(node: &TreeNode) -> i32 {
            let mut depth_left = 1;
            let mut depth_right = 1;
            if let Some(ref left) = node.left {
                depth_left = helper(&left.borrow()) + 1
            }
            if let Some(ref right) = node.right {
                depth_right = helper(&right.borrow()) + 1
            }
            if depth_left > depth_right {
                return depth_left;
            }
            return depth_right;
        }
        return helper(self);
    }

    pub fn is_symmetric(&self) -> bool {
        fn helper(
            left: Option<&Rc<RefCell<TreeNode>>>,
            right: Option<&Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if left.is_none() && right.is_none() {
                return true;
            }
            if left.is_none() && right.is_some() {
                return false;
            }
            if left.is_some() && right.is_none() {
                return false;
            }

            if left.unwrap().borrow().val != right.unwrap().borrow().val {
                return false;
            }

            if !helper(
                left.unwrap().borrow().left.as_ref(),
                right.unwrap().borrow().right.as_ref(),
            ) {
                return false;
            }
            return helper(
                left.unwrap().borrow().right.as_ref(),
                right.unwrap().borrow().left.as_ref(),
            );
        }
        return helper(self.left.as_ref(), self.right.as_ref());
    }

    pub fn has_path_sum(&self, target_sum: i32) -> bool {
        let mut target_sum = target_sum;
        fn helper(node: &TreeNode, target_sum: &mut i32) -> bool {
            *target_sum = *target_sum - node.val;
            if *target_sum == 0 && node.left.is_none() && node.right.is_none() {
                return true;
            }
            if let Some(ref left) = node.left {
                if helper(&left.borrow(), target_sum) {
                    return true;
                } else {
                    *target_sum += left.borrow().val;
                }
            }
            if let Some(ref right) = node.right {
                if helper(&right.borrow(), target_sum) {
                    return true;
                } else {
                    *target_sum += right.borrow().val;
                }
            }
            return false;
        }
        return helper(self, &mut target_sum);
    }

    pub fn connect(&self) -> Option<Rc<RefCell<TreeNode>>> {
        return None;
    }

    // 将水平node节点连接
    pub fn connect_with_next(&mut self) {
        let mut level_node = Vec::<Vec<Rc<RefCell<TreeNode>>>>::new();
        fn helper(node: &TreeNode, level_node: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>, depth: usize) {
            if depth == level_node.len() {
                level_node.push(Vec::<Rc<RefCell<TreeNode>>>::new())
            }

            // let mut rows = &level_node[depth];
            if let Some(ref left) = node.left {
                level_node[depth].push(left.clone());
                helper(&left.borrow(), level_node, depth + 1);
            }
            if let Some(ref right) = node.right {
                level_node[depth].push(right.clone());
                helper(&right.borrow(), level_node, depth + 1);
            }
        }
        helper(self, &mut level_node, 0);

        let mut out_idx = 0usize;
        while out_idx < level_node.len() {
            let mut rows = &level_node[out_idx];
            out_idx += 1;
            let mut inner_idx = 0usize;
            if rows.len() <= 1 {
                continue;
            }
            while inner_idx < rows.len() - 1 {
                rows[inner_idx].borrow_mut().next = Some(rows[inner_idx + 1].to_owned());
                inner_idx += 1;
            }
        }
    }

    // pub fn lowest_common_ancestor(
    //     &self,
    //     p: &Rc<RefCell<TreeNode>>,
    //     q: &Rc<RefCell<TreeNode>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     let flag_p = self.find_pos(&p).unwrap();
    //     let flag_q = self.find_pos(&q).unwrap();
    //     match flag_p + flag_q {
    //         0 | 1 => return Some(Rc::new(RefCell::new(*self))),
    //         2 => {
    //             if let Some(ref left) = self.left {
    //                 let lleft = &left.borrow();
    //                 return lleft.lowest_common_ancestor(&p, &q);
    //             }
    //         }
    //         -2 => {
    //             if let Some(ref right) = self.right {
    //                 let rright = &right.borrow();
    //                 return rright.lowest_common_ancestor(&p, &q);
    //             }
    //         }
    //         _ => return None,
    //     }
    //     return None;
    // }

    // // target是否在root树中;None:不在树中，-1：在左子树中；0:在root根节点上；1：在右子树中
    // fn find_pos(&self, target: &Rc<RefCell<TreeNode>>) -> Option<i32> {
    //     if self.val == target.borrow().val {
    //         return Some(0);
    //     }

    //     if let Some(ref left) = self.left {
    //         if let Some(flag) = TreeNode::find_pos(&left.borrow(), target) {
    //             match flag {
    //                 -1 | 0 | 1 => return Some(-1),
    //                 _ => {}
    //             }
    //         }
    //     }

    //     if let Some(ref right) = self.right {
    //         if let Some(flag) = TreeNode::find_pos(&right.borrow(), target) {
    //             match flag {
    //                 -1 | 0 | 1 => return Some(1),
    //                 _ => {}
    //             }
    //         }
    //     }
    //     return None;
    // }
}
