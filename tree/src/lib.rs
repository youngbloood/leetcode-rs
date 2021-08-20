mod node;

use node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            // ref indicates that you want a reference to an unpacked value.
            // Rather than pattern matching, for a reference value, which would be: Some(&node).
            Some(ref node) => {
                // borrow called on our ref to a RefCell gives us access to a reference to T and its methods, in this case preorder.
                node.borrow().preorder()
            }
            None => {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        return Some(Rc::new(RefCell::new(root)));
        // return root;
    }

    #[test]
    pub fn test_preorder_traversal() {
        let tree = gen_tree();
        let list = Solution::preorder_traversal(tree);
        println!("list = {:?}", list);
    }
}
