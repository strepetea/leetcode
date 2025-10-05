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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
}

pub fn get_list_length(list: Option<Box<ListNode>>) -> i32 {
    add_if_next(list, 0)
}
pub fn add_if_next(list: Option<Box<ListNode>>, mut result: i32) -> i32 {
    let list = list.unwrap();
    result += 1;
    if list.next == None {
        return result;
    } else {
        return add_if_next(list.next, result);
    }
}

pub fn convert_to_int(list: Option<Box<ListNode>>) -> i32 {
    let mut result = 0;
    let mut n = get_list_length(list.clone());
    let mut current = list.unwrap().val;
    while n >= 0 {
        result += current * 10_i32.pow(n as u32);
        n -= 1;
        current = list.unwrap().next.unwrap().val;
    }
    result
}

fn main() {}
