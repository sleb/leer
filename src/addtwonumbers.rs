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
    let l1 = l1.expect("l1 was empty");
    let l2 = l2.expect("l2 was empty");

    let first_sum = l1.val + l2.val;
    let mut result = ListNode::new(first_sum % 10);
    let mut carry = first_sum / 10;

    let mut cursor1 = l1.next;
    let mut cursor2 = l2.next;
    let mut current = &mut result;
    while let Some(n1) = cursor1 {
        let mut sum = n1.val + carry;
        if let Some(n2) = cursor2 {
            sum += n2.val;
            cursor2 = n2.next
        }

        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        carry = sum / 10;
        cursor1 = n1.next;
    }

    while let Some(n2) = cursor2 {
        let sum = n2.val + carry;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        carry = sum / 10;
        cursor2 = n2.next;
    }

    if carry != 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    Some(Box::new(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new(n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            return Some(Box::new(ListNode::new(0)));
        }

        let mut digits = Vec::new();
        let mut n = n;

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let mut head = ListNode::new(digits.pop().unwrap());
        while !digits.is_empty() {
            let tmp = head;
            head = ListNode::new(digits.pop().unwrap());
            head.next = Some(Box::new(tmp));
        }

        Some(Box::new(head))
    }

    #[test]
    fn example1() {
        let l1 = new(342);
        let l2 = new(465);

        assert_eq!(new(807), add_two_numbers(l1, l2));
    }

    #[test]
    fn example2() {
        let l1 = new(0);
        let l2 = new(0);

        assert_eq!(new(0), add_two_numbers(l1, l2));
    }

    #[test]
    fn example3() {
        let l1 = new(9999999);
        let l2 = new(9999);

        assert_eq!(new(10009998), add_two_numbers(l1, l2));
    }
}
