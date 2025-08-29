//! You are given two non-empty linked lists representing two non-negative integers.
//! The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    /// Time complexity : O(max(m,n)). Assume that m and n represents the length of l1 and l2 respectively.
    /// Space complexity : O(1). The length of the new list is at most max(m,n)+1 However, we don't count
    /// the answer as part of the space complexity.
    /// Because no matter what algorithm you choose, you must produce that output; it’s considered necessary, not overhead.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // pointers to walking through the input lists
        let mut p = l1;
        let mut q = l2;

        // to carry the overflows 7+8=15 -> carry = 1
        let mut carry = 0;

        // utility for building the results
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while p.is_some() || q.is_some() || carry != 0 {
            let m = p.as_ref().map_or(0, |node| node.val);
            let n = q.as_ref().map_or(0, |node| node.val);

            let sum = m + n + carry;
            carry = sum / 10;

            // create a new node, attach it to the result and move tail forward.
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();

            if let Some(node) = q {
                q = node.next;
            }
            if let Some(node) = p {
                p = node.next;
            }
        }

        dummy.next
    }

    /// Convert a linked list to a vec. "list" is the head of linked-list.
    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();

        // current is a pointer for traversing the list, starts at the head!
        let mut current = list;
        while let Some(node) = current {
            v.push(node.val);
            current = node.next
        }
        v
    }

    /// Build a linked list from a vec. Building is done from head to tail.
    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);

        // tail is used to update the tail node! at the beginning it is the head.
        let mut tail = &mut dummy;

        for value in v {
            tail.next = Some(Box::new(ListNode::new(value)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }

    /// Build a linked list from a vec. Building is done from tail to head.
    fn from_vec_2(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &value in v.iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }

        head
    }
}

fn main() {
    let v = vec![1, 8, 9, 3];
    let list = Solution::from_vec(v);
    dbg!(&list);
    let v = Solution::to_vec(list);
    dbg!(v);
}
