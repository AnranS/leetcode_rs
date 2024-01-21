use crate::uitls::linked_list::ListNode;

// Definition for singly-linked list.
struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut res;
        let mut overflow = false;
        while l1.is_some() || l2.is_some() || overflow {
            let l1_v = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };
            let l2_v = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };
            let sum = l1_v + l2_v + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        res.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use crate::uitls::linked_list::to_list;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
