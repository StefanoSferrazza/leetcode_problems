
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

// 1 3 5 7
// 4 6 8



pub struct Solution;
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1,list2){
            (None, None) => { return None; }
            (Some(l), None) | (None, Some(l)) => { return Some(l); }
            (Some(l1), Some(l2)) => {
                let first_pointer;
                if l1.val<l2.val{
                    match l1.next{
                        // l1.val<l2.val<l1_next.val
                        Some(l1_next) if l1_next.val>l2.val => {
                            // l1_next_temp = l1.next;
                            l2_next_temp = l2.next;

                            l2.next = l1.next;
                            l1.next = l2;

                            l2.next = l1_next_temp;

                            l2 = l2.next;
                        }
                        Some(l1_next) if l1_next<=l2.val => {

                        }
                        None => {
                            l1.next = l2;
                        }
                    }
                    first_pointer=&l1;
                }
                else{
                    first_pointer=&l2;
                }
                while


                if l1.val > l2.val{
                    if l2.next < l1.val
                    return l2;
                }

                return *first_pointer;
            }
        }
        return None;
    }
}

fn main() {
    println!("Hello, world!");
}
