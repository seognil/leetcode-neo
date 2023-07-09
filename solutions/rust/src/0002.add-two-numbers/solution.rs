// Created by seognil LC at 2023/07/09 22:53
// leetgo: 1.3.3
// https://leetcode.cn/problems/add-two-numbers/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// * [ '0 ms', '100%', '2 MB', '92%' ]

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2, mut carry) = (l1, l2, 0);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let val = match l1 {
                Some(n) => {
                    l1 = n.next;
                    n.val
                }
                None => 0,
            } + match l2 {
                Some(n) => {
                    l2 = n.next;
                    n.val
                }
                None => 0,
            } + carry;

            carry = val / 10;
            tail.next = Some(Box::new(ListNode::new(val % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        head.next
    }
}

// @lc code=end

fn main() -> Result<()> {
    let l1: LinkedList = deserialize(&read_line()?)?;
    let l2: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::add_two_numbers(l1.into(), l2.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
