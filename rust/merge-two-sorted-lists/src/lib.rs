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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => Some(Box::new(ListNode{
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2))
                })),
                false => Some(Box::new(ListNode{
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next)
                }))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{Solution, ListNode};

    #[test]
    fn test0() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![1,2,4]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![1,3,4]);

        assert!(input_2 != input_1);
    }

    #[test]
    fn test1() {
        let input_1: Option<Box<ListNode>> = ListNode::from_vec(vec![1,2,4]);
        let input_2: Option<Box<ListNode>> = ListNode::from_vec(vec![1,3,4]);

        let expected: Option<Box<ListNode>> = ListNode::from_vec(vec![1, 1, 2, 3, 4]);

        assert_eq!(expected, Solution::merge_two_lists(input_1, input_2));
    }

}
