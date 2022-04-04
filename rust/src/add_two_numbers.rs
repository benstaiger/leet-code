// You are given two non-empty linked lists representing two non-negative
// integers. The digits are stored in reverse order, and each of their nodes
// contains a single digit. Add the two numbers and return the sum as a linked
// list.
//
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    fn create_num_stack(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut list = list;
        let mut result = Vec::new();
        loop {
            match list {
                Some(node) => {
                    result.push(node.val);
                    list = node.next;
                }
                None => { break; }
            }
        }
        result
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;

        let mut head: Option<Box<ListNode>> = None;
        let mut tail: &mut Option<Box<ListNode>> = &mut head;
        let mut remainder = 0;
        while p1.is_some() || p2.is_some() || remainder > 0 {
            let v1 = if let Some(node) = p1 { p1 = node.next; node.val } else { 0 };
            let v2 = if let Some(node) = p2 { p2 = node.next; node.val } else { 0 };
            let sum = v1 + v2 + remainder;
            let v = sum % 10;
            remainder = if sum >= 10 { 1 } else { 0 };

            let mut next = Box::new(ListNode::new(v));
            *tail = Some(next);
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_list(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;
        for &v in nums.iter().rev() {
            let mut next = ListNode::new(v);
            next.next = result;
            result = Some(Box::new(next));
        }
        result
    }
    
    #[test]
    fn example1() {
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // Output: [7,0,8]
        // Explanation: 342 + 465 = 807.
        let l1 = create_list(&vec![2, 4, 3]);
        let l2 = create_list(&vec![5, 6, 4]);
        let expected = create_list(&vec![7, 0, 8]);
        println!("l1: {:?}", &l1);
        println!("l2: {:?}", &l2);
        let result = Solution::add_two_numbers(l1, l2);
        println!("{:?}\n{:?}", &expected, &result);
        assert!(result == expected);
    }

    #[test]
    fn example2() {
        // Input: l1 = [0], l2 = [0]
        // Output: [0]
        let l1 = create_list(&vec![0]);
        let l2 = create_list(&vec![0]);
        let expected = create_list(&vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert!(result == expected);
    }

    #[test]
    fn example3() {
        // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // Output: [8,9,9,9,0,0,0,1]
        let l1 = create_list(&vec![9,9,9,9,9,9,9]);
        let l2 = create_list(&vec![9,9,9,9]);
        let expected = create_list(&vec![8,9,9,9,0,0,0,1]);
        let result = Solution::add_two_numbers(l1, l2);
        println!("{:?}\n{:?}", &expected, &result);
        assert!(result == expected);
    }
}