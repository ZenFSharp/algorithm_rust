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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list = Some(Box::new(ListNode::new(0)));
        let mut node = &mut list.as_mut().unwrap().next;
        let mut l1 = l1;
        let mut l2 = l2;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            if n1.val < n2.val {
                *node = l1;
                l1 = (*node).as_mut().unwrap().next.take();
            } else {
                *node = l2;
                l2 = (*node).as_mut().unwrap().next.take();
            }
            node = &mut (*node).as_mut().unwrap().next
        }

        *node = l1.or(l2);

        list.unwrap().next.take()
    }
}

fn main() {
    let result = Solution::merge_two_lists(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(3))),
        })),
        Some(Box::new(ListNode::new(2))),
    );
    println!("{:?}", result);
}
