/**
single linked list
*/
use crate::node::NodeSingle as Node;

/**
https://leetcode.com/explore/learn/card/linked-list/209/singly-linked-list/1290/

Design Linked List

Solution
Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.

Implement the LinkListSingle class:

LinkListSingle() Initializes the LinkListSingle object.
int get(int index) Get the value of the index^th node in the linked list. If the index is invalid, return -1.
void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
void addAtTail(int val) Append a node of value val as the last element of the linked list.
void addAtIndex(int index, int val) Add a node of value val before the index^th node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
void deleteAtIndex(int index) Delete the index^th node in the linked list, if the index is valid.


Example 1:

Input
["LinkListSingle", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
[[], [1], [3], [1, 2], [1], [1], [1]]
Output
[null, null, null, null, 2, null, 3]

Explanation
LinkListSingle myLinkedList = new LinkListSingle();
myLinkedList.addAtHead(1);
myLinkedList.addAtTail(3);
myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
myLinkedList.get(1);              // return 2
myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
myLinkedList.get(1);              // return 3


Constraints:

0 <= index, val <= 1000
Please do not use the built-in LinkedList library.
At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.

*/

#[derive(Default, Debug)]
pub struct LinkListSingle {
    length: usize,
    head: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LinkListSingle {
    /** Initialize your data structure here. */
    fn new() -> Self {
        return Default::default();
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        let mut cur = match self.head {
            Some(ref head) => head,
            None => return -1,
        };
        let mut idx_cur = 0;
        while idx_cur < index {
            if let Some(ref next) = cur.next {
                cur = next;
                idx_cur += 1;
            } else {
                return -1;
            }
        }
        return cur.val;
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
        self.length += 1;
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut cur = match self.head {
            Some(ref mut head) => head,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };
        while let Some(ref mut next) = cur.next {
            cur = next;
        }

        cur.next = Some(Box::new(Node { val, next: None }));
        self.length += 1;
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val,
            next: self.head.take(),
        });

        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }

        cur.next = Some(Box::new(Node {
            val,
            next: cur.next.take(),
        }));

        self.head = dummy_head.next;
        self.length += 1;
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });

        let mut idx = 0;
        let mut cur = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = cur.next {
                cur = next;
            }
            idx += 1;
        }
        cur.next = cur.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
        self.length -= 1;
    }

    /**
    https://leetcode.com/explore/learn/card/linked-list/214/two-pointer-technique/1296/

    Remove Nth Node From End of List

    Solution
    Given the head of a linked list, remove the nth node from the end of the list and return its head.



    Example 1:
    1 -> 2 -> 3 -> 4 -> 5
             ||
             \/
    1 -> 2 -> 3 ------> 5

    Input: head = [1,2,3,4,5], n = 2
    Output: [1,2,3,5]

    Example 2:
    Input: head = [1], n = 1
    Output: []

    Example 3:
    Input: head = [1,2], n = 1
    Output: [1]


    Constraints:

    The number of nodes in the list is sz.
    1 <= sz <= 30
    0 <= Node.val <= 100
    1 <= n <= sz


    Follow up: Could you do this in one pass?

    Hide Hint #1
    Maintain two pointers and update one with a delay of n steps.
        */
    pub fn remove_nth_from_end(head: Option<Box<Node>>, n: i32) -> Option<Box<Node>> {
        todo!();
        // let mut ptr1 = &head;
        // let mut ptr2 = &head;

        // let mut idx = 0;

        // while let Some(node2) = ptr2 {
        //     ptr2 = &node2.next;
        //     idx += 1;
        //     if idx >= n {
        //         if let Some(node1) = ptr1 {
        //             ptr1 = &node1.next;
        //         }
        //     }
        // }

        // let mut cur = &mut ptr1.unwrap();
        // cur.next = cur.next.take().and_then(|a| a.next);
        // // ptr1.unwrap().next = ptr1.unwrap().next.unwrap().next;
        // return head;
    }

    pub fn remove_nth_from_end_self(&mut self, n: i32) {
        self.head = Self::remove_nth_from_end(self.head.take(), n)
    }

    // pub fn reverse(&mut self) {
    //     let mut pre = &self.head;
    //     let mut midd = pre.as_ref().unwrap().next;
    //     let mut post = midd.as_ref().unwrap().next;
    // }
}

/**
 * Your LinkListSingle object will be instantiated and called as such:
 * let obj = LinkListSingle::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_linklist_single() {
        let mut ll = LinkListSingle::new();
        ll.add_at_head(1);
        println!("ll = {:?}", ll);
        ll.add_at_tail(2);
        println!("ll = {:?}", ll);
        ll.add_at_head(3);
        println!("ll = {:?}", ll);
        ll.delete_at_index(2);
        println!("ll = {:?}", ll);
        ll.add_at_index(2, 11);
        println!("ll = {:?}", ll);
        ll.remove_nth_from_end_self(2);
        println!("ll = {:?}", ll);
    }
}
