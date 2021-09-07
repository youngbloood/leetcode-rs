mod node;

use node::TreeNode;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct Solution;

impl Solution {
    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/133/conclusion/943/

    Construct Binary Tree from Preorder and Inorder Traversal

    Solution
    Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.



    Example 1:
            3
          /   \
         9     20
              /  \
             15   7


    Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
    Output: [3,9,20,null,null,15,7]
    Example 2:

    Input: preorder = [-1], inorder = [-1]
    Output: [-1]


    Constraints:

    1 <= preorder.length <= 3000
    inorder.length == preorder.length
    -3000 <= preorder[i], inorder[i] <= 3000
    preorder and inorder consist of unique values.
    Each value of inorder also appears in preorder.
    preorder is guaranteed to be the preorder traversal of the tree.
    inorder is guaranteed to be the inorder traversal of the tree.

        */
    pub fn new_tree_node(preorder: &Vec<i32>, inorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return TreeNode::new_with_pre_and_in(preorder, inorder);
    }
    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/134/traverse-a-tree/928/

    Binary Tree Preorder Traversal

    Solution
    Given the root of a binary tree, return the preorder traversal of its nodes' values.



    Example 1:
            1
             \
              2
             /
            3

    Input: root = [1,null,2,3]
    Output: [1,2,3]

    Example 2:

    Input: root = []
    Output: []

    Example 3:

    Input: root = [1]
    Output: [1]

    Example 4:
            1
           /
          2

    Input: root = [1,2]
    Output: [1,2]

    Example 5:
           1
            \
             2

    Input: root = [1,null,2]
    Output: [1,2]


    Constraints:

    The number of nodes in the tree is in the range [0, 100].
    -100 <= Node.val <= 100

    Follow up: Recursive solution is trivial, could you do it iteratively?
        */
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            // ref indicates that you want a reference to an unpacked value.
            // Rather than pattern matching, for a reference value, which would be: Some(&node).
            Some(ref node) => {
                // borrow called on our ref to a RefCell gives us access to a reference to T and its methods, in this case preorder.
                let nd = node.borrow();
                return nd.preorder();
            }
            None => return vec![],
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/134/traverse-a-tree/929/

    Binary Tree Inorder Traversal

    Solution
    Given the root of a binary tree, return the inorder traversal of its nodes' values.



    Example 1:
            1
             \
              2
             /
            3

    Input: root = [1,null,2,3]
    Output: [1,3,2]

    Example 2:
    Input: root = []
    Output: []

    Example 3:
    Input: root = [1]
    Output: [1]

    Example 4:
            1
           /
          2

    Input: root = [1,2]
    Output: [2,1]

    Example 5:
            1
             \
              2
    Input: root = [1,null,2]
    Output: [1,2]


    Constraints:

    The number of nodes in the tree is in the range [0, 100].
    -100 <= Node.val <= 100


    Follow up: Recursive solution is trivial, could you do it iteratively?
        */
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(ref node) => {
                return node.borrow().inorder();
            }
            None => return vec![],
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/134/traverse-a-tree/930/

    Binary Tree Postorder Traversal

    Given the root of a binary tree, return the postorder traversal of its nodes' values.



    Example 1:
            1
             \
              2
             /
            3
    Input: root = [1,null,2,3]
    Output: [3,2,1]

    Example 2:
    Input: root = []
    Output: []

    Example 3:
    Input: root = [1]
    Output: [1]

    Example 4:
            1
           /
          2
    Input: root = [1,2]
    Output: [2,1]

    Example 5:
            1
             \
              2
    Input: root = [1,null,2]
    Output: [2,1]

    Constraints:

    The number of the nodes in the tree is in the range [0, 100].
    -100 <= Node.val <= 100


    Follow up: Recursive solution is trivial, could you do it iteratively?
        */
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(ref node) => {
                return node.borrow().postorder();
            }
            None => {
                return vec![];
            }
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/134/traverse-a-tree/931/

    Binary Tree Level Order Traversal

    Solution
    Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).



    Example 1:
            3
           / \
          9  20
             / \
            15  7
    Input: root = [3,9,20,null,null,15,7]
    Output: [[3],[9,20],[15,7]]
    Example 2:

    Input: root = [1]
    Output: [[1]]
    Example 3:

    Input: root = []
    Output: []


    Constraints:

    The number of nodes in the tree is in the range [0, 2000].
    -1000 <= Node.val <= 1000
        */
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            Some(ref node) => {
                return node.borrow().level_order();
            }
            None => {
                return vec![];
            }
        }
    }
    /*
    https://leetcode.com/explore/learn/card/data-structure-tree/17/solve-problems-recursively/535/


    Maximum Depth of Binary Tree

    Solution
    Given the root of a binary tree, return its maximum depth.

    A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.



    Example 1:
                3
               /  \
              9   20
                 /  \
                15   7

    Input: root = [3,9,20,null,null,15,7]
    Output: 3

    Example 2:
    Input: root = [1,null,2]
    Output: 2

    Example 3:
    Input: root = []
    Output: 0

    Example 4:
    Input: root = [0]
    Output: 1


    Constraints:

    The number of nodes in the tree is in the range [0, 10^4].
    -100 <= Node.val <= 100
         */
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(ref node) => {
                return node.borrow().max_depth();
            }
            None => {
                return 0;
            }
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/17/solve-problems-recursively/536/

    Symmetric Tree

    Solution
    Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).



    Example 1:
             1
           / | \
          2  |  2
         / \ | / \
        3   4|4   3
    Input: root = [1,2,2,3,4,4,3]
    Output: true

    Example 2:
             1
           /   \
          2     2
           \     \
            3     3


    Input: root = [1,2,2,null,3,null,3]
    Output: false


    Constraints:

    The number of nodes in the tree is in the range [1, 1000].
    -100 <= Node.val <= 100

    Follow up: Could you solve it both recursively and iteratively?
    */
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(ref node) => {
                return node.borrow().is_symmetric();
            }
            None => {
                return false;
            }
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/17/solve-problems-recursively/537/

    Path Sum

    Solution
    Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

    A leaf is a node with no children.



    Example 1:
                            5
                      /           \
                     4             8
                   /             /   \
                  11           13     4
                /   \                   \
              7      2                   1

    preorder: 5,4,11,7,2,8,13,4,1
    inorder:  7,11,2,4,5,13,8,4,1

    Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
    Output: true

    Example 2:
               1
              /  \
             2    3

    Input: root = [1,2,3], targetSum = 5
    Output: false
    Example 3:

    Input: root = [1,2], targetSum = 0
    Output: false


    Constraints:

    The number of nodes in the tree is in the range [0, 5000].
    -1000 <= Node.val <= 1000
    -1000 <= targetSum <= 1000
        */

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                return node.borrow().has_path_sum(target_sum);
            }
            None => {
                return false;
            }
        }
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/133/conclusion/942/

    Construct Binary Tree from Inorder and Postorder Traversal

    Solution
    Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.



    Example 1:
            3
          /   \
         9     20
              /  \
             15   7

    Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
    Output: [3,9,20,null,null,15,7]

    Example 2:
    Input: inorder = [-1], postorder = [-1]
    Output: [-1]


    Constraints:

    1 <= inorder.length <= 3000
    postorder.length == inorder.length
    -3000 <= inorder[i], postorder[i] <= 3000
    inorder and postorder consist of unique values.
    Each value of postorder also appears in inorder.
    inorder is guaranteed to be the inorder traversal of the tree.
    postorder is guaranteed to be the postorder traversal of the tree.
        */
    pub fn build_tree_with_in_and_post(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return TreeNode::new_with_in_and_post(&inorder, &postorder);
    }

    /**
    https://leetcode.com/explore/learn/card/data-structure-tree/133/conclusion/932/


    Lowest Common Ancestor of a Binary Tree

    Solution
    Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

    According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

    Example 1:
            3
         /     \
        5       1
       / \     / \
      6   2   0   8
         / \
        7   4

    Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
    Output: 3
    Explanation: The LCA of nodes 5 and 1 is 3.

    Example 2:
            3
         /     \
        5       1
       / \     / \
      6   2   0   8
         / \
        7   4

    Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
    Output: 5
    Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.

    Example 3:
    Input: root = [1,2], p = 1, q = 2
    Output: 1

    Constraints:

    The number of nodes in the tree is in the range [2, 10^5].
    -10^9 <= Node.val <= 10^9
    All Node.val are unique.
    p != q
    p and q will exist in the tree.
        */
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return None;
    }

    pub fn connect_with_next(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(ref mut node) => node.borrow_mut().connect_with_next(),
            None => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert::{AssertType, Judge};
    use std::fmt::Debug;

    fn gen_tree(preorder: &Vec<i32>, inorder: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Solution::new_tree_node(preorder, inorder);
    }

    /**
            3
          /   \
         9     20
       /  \    /  \
      8    5  15   7
    */
    fn gen_normal_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let preorder = vec![3, 9, 8, 5, 20, 15, 7];
        let inorder = vec![8, 9, 5, 3, 15, 20, 7];
        return gen_tree(&preorder, &inorder);
    }

    #[test]
    pub fn test_preorder_traversal() {
        let test_example = vec![(vec![3, 9, 8, 5, 20, 15, 7], vec![8, 9, 5, 3, 15, 20, 7])];

        for (preorder, inorder) in test_example {
            let root = gen_tree(&preorder, &inorder);
            let list = Solution::preorder_traversal(root);
            let judge = Judge::<Vec<i32>>::new(AssertType::Eq, preorder);
            judge.assert(list);
        }
    }

    #[test]
    pub fn test_gen_tree() {
        let test_example = vec![(vec![3, 9, 8, 5, 20, 15, 7], vec![8, 9, 5, 3, 15, 20, 7])];

        for (preorder, inorder) in test_example {
            let root = gen_tree(&preorder, &inorder);
            println!("root = {:?}", root);
        }
    }

    #[test]
    pub fn test_level_order() {
        let test_example = vec![(
            vec![3, 9, 8, 5, 20, 15, 7],
            vec![8, 9, 5, 3, 15, 20, 7],
            Judge::<Vec<Vec<i32>>>::new(
                AssertType::Eq,
                vec![vec![3], vec![9, 20], vec![8, 5, 15, 7]],
            ),
        )];

        for (preorder, inorder, judge) in test_example {
            let root = gen_tree(&preorder, &inorder);
            let level_order = Solution::level_order(root);
            judge.assert(level_order);
        }
    }

    #[test]
    pub fn test_max_depth() {
        let test_example = vec![(
            vec![3, 9, 8, 5, 20, 15, 7],
            vec![8, 9, 5, 3, 15, 20, 7],
            Judge::<i32>::new(AssertType::Eq, 3),
        )];

        for (preorder, inorder, judge) in test_example {
            let root = gen_tree(&preorder, &inorder);
            let depth = Solution::max_depth(root);
            judge.assert(depth);
        }
    }

    #[test]
    pub fn test_is_symmetric() {
        let test_example = vec![
            (
                vec![3, 9, 8, 5, 20, 15, 7],
                vec![8, 9, 5, 3, 15, 20, 7],
                Judge::<bool>::new(AssertType::Eq, false),
            ),
            (
                vec![1, 2, 3, 4, 2, 4, 3],
                vec![3, 2, 4, 1, 4, 2, 3],
                Judge::<bool>::new(AssertType::Eq, true),
            ),
            (
                vec![1, 2, 3, 2, 3],
                vec![3, 2, 1, 3, 2],
                Judge::<bool>::new(AssertType::Eq, false),
            ),
        ];

        for (preorder, inorder, judge) in test_example {
            let root = gen_tree(&preorder, &inorder);
            let symmetric = Solution::is_symmetric(root);
            judge.assert(symmetric);
            // judge.assert(None);
        }
    }

    #[test]
    pub fn test_has_path_sum() {
        let test_example = vec![
            (
                vec![5, 4, 11, 7, 2, 8, 13, 4, 1],
                vec![7, 11, 2, 4, 5, 13, 8, 4, 1],
                22,
                Judge::<bool>::new(AssertType::Eq, true),
            ),
            (
                vec![1, 2],
                vec![1, 2],
                1,
                Judge::<bool>::new(AssertType::Eq, false),
            ),
        ];

        for (preorder, inorder, target_sum, judge) in test_example {
            let root = gen_tree(&preorder, &inorder);
            let has_path_sum = Solution::has_path_sum(root, target_sum);
            judge.assert(has_path_sum);
        }
    }

    #[test]
    pub fn test_build_tree_with_in_and_post() {
        let test_example = vec![(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])];

        for (inorder, postorder) in test_example {
            let root = Solution::build_tree_with_in_and_post(inorder, postorder);
            println!("root = {:?}", root);
        }
    }

    #[test]
    pub fn test_connect_with_next() {
        let mut root = gen_normal_tree();
        Solution::connect_with_next(&mut root);
        println!("root = {:?}", root);

        let preorder = vec![1, 2, 3];
        let inorder = vec![2, 1, 3];
        let mut root = gen_tree(&preorder, &inorder);
        Solution::connect_with_next(&mut root);
        println!("root = {:?}", root);
    }
}
