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

    fn from_vec(mut values: Vec<i32>) -> Option<Box<Self>>{
        if values.is_empty() {
            return None
        }

        let x = values.pop().unwrap();
        let mut list_node = ListNode::new(x);

        while !values.is_empty() {    
            let x = values.pop().unwrap();
            list_node = ListNode{ val: x, next: Some(Box::new(list_node)) };
        }

        Some(Box::new(list_node))
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("l1: {:?}", l1);
        println!("l2: {:?}", l2);
        println!("");

        match (l1, l2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(l1), Some(l2)) => match l1.val + l2.val >= 10 {
                false => Some(Box::new(
                    ListNode { val: l1.val + l2.val, next: Solution::add_two_numbers(l1.next, l2.next) }
                )),
                true => {
                    let decimal_sum = (l1.val + l2.val) % 10;
                    let l1_next = l1.next;
                    let l2_next = l2.next;

                    match (l1_next, l2_next) {
                        (None, None) => Some(Box::new(
                            ListNode{ val: decimal_sum, next: Some(Box::new(ListNode::new(1)))}
                        )),
                        (Some(l1_next), None) => Some(Box::new(
                            ListNode { val: decimal_sum, next: Solution::add_two_numbers(
                                Some(Box::new(ListNode { val: l1_next.val, next: l1_next.next })),
                                Some(Box::new(ListNode::new(1)))
                            ) }
                        )),
                        (None, Some(l2_next)) => Some(Box::new(
                            ListNode { val: decimal_sum, next: Solution::add_two_numbers(
                                Some(Box::new(ListNode::new(1))),
                                Some(Box::new(ListNode { val: l2_next.val, next: l2_next.next }))
                            ) }
                        )),
                        (Some(l1_next), Some(l2_next)) => Some(Box::new(
                            ListNode { val: decimal_sum, next: Solution::add_two_numbers(
                                Some(l1_next),
                                Some(Box::new(ListNode { val: l2_next.val + 1, next: l2_next.next }))) }
                        ))
                    }
                }
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use crate::{Solution, ListNode};

    #[test]
    fn test0() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![2,4]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![5,6]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![7,0,1]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

    #[test]
    fn test1() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![2,4,3]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![5,6,4]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![7,0,8]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

    #[test]
    fn test2() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![2]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![5]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![7]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

    #[test]
    fn test3() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![0]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![0]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![0]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

    #[test]
    fn test4() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![9,9]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![9,9, 9]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![8,9,0,1]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

    #[test]
    fn test5() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![9,9,9,9,9,9,9]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![9,9,9,9]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![8,9,9,9,0,0,0,1]);

        assert_eq!(expected, Solution::add_two_numbers(input_1, input_2));
    }

}
