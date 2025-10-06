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

pub fn len(list: &ListNode) -> i32 {
    let mut current = &list.next;
    let mut res: i32 = 1;
    while let Some(node) = current {
        res += 1;
        current = &node.next;
    }
    res
}

pub fn to_int(list: &Option<Box<ListNode>>) -> i32 {
    let mut current = list.as_ref();
    let mut res: i32 = 0;
    let mut multiplier = 1;
    while let Some(node) = current {
        res += node.val * multiplier;
        multiplier *= 10;
        current = node.next.as_ref();
    }
    res
}

pub fn to_list(sum: i32) -> Option<Box<ListNode>> {
    let sum = Vec::from(sum.to_string());
    let sum: Vec<i32> = sum.iter().map(|&x| (x - b'0') as i32).collect();
    let mut head = Box::new(ListNode::new(*sum.last().unwrap()));
    let mut current = &mut head;
    for digit in sum.iter().rev().skip(1) {
        current.next = Some(Box::new(ListNode::new(*digit)));
        current = current.next.as_mut()?;
    }
    Some(head)
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let sum = to_int(&l1) + to_int(&l2);
    to_list(sum)
}

fn main() {}
