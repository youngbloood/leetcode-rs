use crate::node::NodeSingleRef as Node;
use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, ops::Deref};

/**
Linked List Cycle:
https://leetcode.com/explore/learn/card/linked-list/214/two-pointer-technique/1212/


Given head, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.

Return true if there is a cycle in the linked list. Otherwise, return false.



Example 1:

3 -> 2 -> 0 -> -4
    /|\         |
     |__________|

Input: head = [3,2,0,-4], pos = 1
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).


Example 2:

     1 -> 2
    /|\   |
     |____|

Input: head = [1,2], pos = 0
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
Example 3:


Input: head = [1], pos = -1
Output: false
Explanation: There is no cycle in the linked list.


Constraints:

The number of the nodes in the list is in the range [0, 10^4].
-10^5 <= Node.val <= 10^5
pos is -1 or a valid index in the linked-list.


Follow up: Can you solve it using O(1) (i.e. constant) memory?

 */

#[derive(Debug, Default)]
struct LinkListCycle {
    head: Option<Rc<RefCell<Node>>>,
}

impl LinkListCycle {
    pub fn new() -> Self {
        return Default::default();
    }

    // pub fn has_cycle(&self) -> bool {
    //     let mut cur = match self.head {
    //         Some(ref node) => node,
    //         None => {
    //             return false;
    //         }
    //     };

    //     let mut map = HashMap::new();
    //     while !map.contains_key(cur.clone()) {
    //         map.insert(&cur, 1);
    //         if let Some(ref node) = cur.next {
    //             cur = node;
    //         } else {
    //             return false;
    //         }
    //     }
    //     return false;
    // }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Rc::new(RefCell::new(Node {
            val,
            next: self.head.take(),
        })));
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut head) => head,
            None => {
                self.head = Some(Rc::new(RefCell::new(Node { val, next: None })));
                return;
            }
        };

        // let cur1 = cur.clone().as_ref().borrow().next;
        // while let Some(ref mut node) = cur.clone().deref().get_mut().next {
        //     cur = node;
        // }
        cur.deref().borrow_mut().next = Some(Rc::new(RefCell::new(Node { val, next: None })));
    }

    // /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    // fn add_at_index(&mut self, index: i32, val: i32) {
    //     let mut dummy_head = Box::new(Node {
    //         val,
    //         next: self.head.take(),
    //     });

    //     let mut idx = 0;
    //     let mut cur = &mut dummy_head;
    //     while idx < index {
    //         if let Some(ref mut next) = cur.next {
    //             cur = next;
    //         }
    //         idx += 1;
    //     }

    //     cur.next = Some(Box::new(Node {
    //         val,
    //         next: cur.next.take(),
    //     }));

    //     self.head = dummy_head.next;
    // }

    // /** Delete the index-th node in the linked list, if the index is valid. */
    // fn delete_at_index(&mut self, index: i32) {
    //     let mut dummy_head = Box::new(Node {
    //         val: 0,
    //         next: self.head.take(),
    //     });

    //     let mut idx = 0;
    //     let mut cur = &mut dummy_head;
    //     while idx < index {
    //         if let Some(ref mut next) = cur.next {
    //             cur = next;
    //         }
    //         idx += 1;
    //     }
    //     cur.next = cur.next.take().and_then(|a| a.next);
    //     self.head = dummy_head.next;
    // }

    // fn set_cycle(&mut self, index: i32) {
    //     if index < 0 {
    //         return;
    //     }

    //     let mut cur = &self.head;
    //     let mut cycle_node = &self.head;
    //     let mut idx = 0;
    //     while let Some(node) = cur {
    //         if idx < index {
    //             cycle_node = cur;
    //         }
    //         cur = &node.next;
    //         idx += 1;
    //     }
    //     if idx < index {
    //         return;
    //     }
    //     cur.unwrap().next = cycle_node.as_ref().as_ref();
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linklist_cycle() {
        // let mut ll = LinkListCycle::new();
        // ll.add_at_head(1);
        // println!("ll = {:?}", ll);
        // ll.add_at_tail(2);
        // println!("ll = {:?}", ll);
        // ll.add_at_head(3);
        // println!("ll = {:?}", ll);
        // ll.delete_at_index(2);
        // println!("ll = {:?}", ll);
        // ll.add_at_index(2, 11);
        // println!("ll = {:?}", ll);
        // ll.set_cycle(1);
        // println!("ll = {:?}", ll);
    }
}
