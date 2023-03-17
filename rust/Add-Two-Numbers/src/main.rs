use std::borrow::Borrow;

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

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (result, carryover) = Solution::sum(l1, l2, 0);
        return result;
    }

    pub fn sum(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carryover: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        match (l1, l2) {
            (Some(list1), Some(list2)) => {
                let mut new_carryover = 0;
                let mut sum = list1.val + list2.val + carryover;
                if sum >= 10 {
                    new_carryover = 1;
                    sum = sum - 10;
                }
                let (next, new_carryover) = Solution::sum(list1.next, list2.next, new_carryover);
                let result = Box::new(ListNode {
                    val: sum,
                    next: next,
                });
                (Some(result), new_carryover)
            }
            (Some(list), None) | (None, Some(list)) => {
                let mut new_carryover = 0;
                let mut sum = list.val + carryover;
                if sum >= 10 {
                    new_carryover = 1;
                    sum = sum - 10;
                }
                let (next, new_carryover) = Solution::sum(list.next, None, new_carryover);
                let result = Box::new(ListNode {
                    val: sum,
                    next: next,
                });
                (Some(result), new_carryover)
            }
            (None, None) => {
                if carryover == 1 {
                    let result = Box::new(ListNode {
                        val: carryover,
                        next: None,
                    });
                    return (Some(result), 0);
                }
                (None, 0)
            }
        }
    }
}

fn main() {
    let res = Solution::add_two_numbers(
        Some(Box::new(ListNode::new(5))),
        Some(Box::new(ListNode::new(5))),
    );
    print_res(res);
}

fn print_res(res: Option<Box<ListNode>>) {
    if let Some(list_node) = res {
        println!("{}", list_node.val);
        print_res(list_node.next);
    }
}
