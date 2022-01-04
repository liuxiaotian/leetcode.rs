/// Definition for singly-linked list.
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

/// Solution for [2. Add Two Numbers](https://leetcode-cn.com/problems/add-two-numbers/)
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut p = &mut head;
        let mut t = (l1, l2, 0);
        loop {
            t = match t {
                (None, None, 0) => break,
                (None, None, sum) => (None, None, sum),
                (Some(t1), None, sum) => (t1.next, None, sum + t1.val),
                (None, Some(t2), sum) => (None, t2.next, sum + t2.val),
                (Some(t1), Some(t2), sum) => (t1.next, t2.next, sum + t1.val + t2.val),
            };
            *p = Some(Box::new(ListNode::new(t.2 % 10)));
            p = &mut p.as_mut().unwrap().next;
            t.2 /= 10;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut p = &mut head;
        for &v in vec.iter() {
            *p = Some(Box::new(ListNode::new(v)));
            p = &mut p.as_mut().unwrap().next;
        }
        head
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            ),
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
