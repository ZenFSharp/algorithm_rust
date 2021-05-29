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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;
        let mut nodes: Vec<ListNode> = Vec::new();

        let mut node1 = &l1;
        let mut node2 = &l2;
        loop {
            let n: ListNode;
            match (node1, node2) {
                (None, None) => {
                    if carry {
                        carry = false;
                        n = ListNode::new(1);
                    } else {
                        break;
                    };
                }
                (None, Some(n1)) | (Some(n1), None) => {
                    let value = if carry { n1.val + 1 } else { n1.val };
                    n = ListNode::new(if value < 10 {
                        carry = false;
                        value
                    } else {
                        carry = true;
                        value - 10
                    });
                }
                (Some(n1), Some(n2)) => {
                    let value = if carry {
                        n1.val + n2.val + 1
                    } else {
                        n1.val + n2.val
                    };
                    n = ListNode::new(if value < 10 {
                        carry = false;
                        value
                    } else {
                        carry = true;
                        value - 10
                    });
                }
            }
            nodes.push(n);
            node1 = match node1 {
                Some(n) => &n.next,
                None => &None,
            };
            node2 = match node2 {
                Some(n) => &n.next,
                None => &None,
            };
        }

        let mut result = None;
        while let Some(top) = nodes.pop() {
            let mut node = top;
            node.next = result;
            result = Some(Box::new(node));
        }
        result
    }
}

fn main() {
    let mut node = ListNode::new(9);
    node.next = Some(Box::new(ListNode::new(9)));
    let result = Solution::add_two_numbers(Some(Box::new(node)), Some(Box::new(ListNode::new(1))));
    println!("{:?}", result);
}
