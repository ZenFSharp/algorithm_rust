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
struct Solution {}
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let result = Some(swap_node(head.unwrap()));
        result
    }
}

fn swap_node(head: Box<ListNode>) -> Box<ListNode> {
    let mut head = head;
    let second = head.next.take();
    match second {
        None => head,
        Some(mut s) => {
            let third = s.next.take();
            head.next = match third {
                None => None,
                Some(t) => Some(swap_node(t)),
            };
            s.next = Some(head);
            s
        }
    }
}

fn main() {
    let node = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        })),
    }));
    let result = Solution::swap_pairs(node);
    println!("{:?}", result);
}
