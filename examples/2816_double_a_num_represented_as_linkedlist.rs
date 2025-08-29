//! You are given the head of a non-empty linked list representing a non-negative integer without leading zeroes.
//! Return the head of the linked list after doubling it.
//!

// TODO: more optimization with trying other solutions from editorial

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
    pub fn double_it_using_stack(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = Self::to_vec(head);

        let mut carry = 0;
        let mut head: Option<Box<ListNode>> = None;

        // You cannot use || in let chain expression
        while !stack.is_empty() || carry != 0 {
            let x = stack.pop().unwrap();
            let mul_result = 2 * x + carry;
            carry = mul_result / 10;

            // Build the list from tail to head
            let mut new_node = Some(Box::new(ListNode::new(mul_result % 10)));
            new_node.as_mut().unwrap().next = head;
            head = new_node;
        }

        head
    }

    pub fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // current is a pointer for traversal
        let mut current = list;

        let mut head = None;
        while let Some(node) = current {
            let mut new_node = ListNode::new(node.val);
            new_node.next = head;
            head = Some(Box::new(new_node));

            current = node.next;
        }
        head
    }

    pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut results = Vec::new();
        let mut current = list;
        while let Some(ref node) = current {
            results.push(node.val);
            current = current.unwrap().next;
        }
        results
    }
}

fn main() {
    let l = ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(0))),
        })),
    };
    // dbg!(Solution::reverse(Some(Box::new(l))));
    dbg!(Solution::double_it_using_stack(Some(Box::new(l))));
}
