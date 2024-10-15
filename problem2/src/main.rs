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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;
        let mut result: Option<Box<ListNode>> = None;
        let mut current = &mut result;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = p1 {
                sum += node.val;
                p1 = node.next;
            }

            if let Some(node) = p2 {
                sum += node.val;
                p2 = node.next;
            }

            carry = sum / 10;
            let new_node = Box::new(ListNode::new(sum % 10));

            *current = Some(new_node);
            current = &mut current.as_mut().unwrap().next;
        }
        result
    }
}

fn main() {
    fn vec_to_list_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut current = &mut head;

        for &val in vec.iter() {
            let new_node = Box::new(ListNode::new(val));
            *current = Some(new_node);
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }

    fn list_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    let l1 = vec_to_list_node(vec![2, 4, 3]);
    let l2 = vec_to_list_node(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);

    let result_vec = list_to_vec(result);

    println!("{:?}", result_vec);
}
