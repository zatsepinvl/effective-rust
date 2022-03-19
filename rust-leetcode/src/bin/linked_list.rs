#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

fn main() {
    let c = None;
    let b = Option::Some(Box::new(ListNode::new(2, c)));
    let a = Option::Some(Box::new(ListNode::new(1, b)));

    let mut head = a;
    let mut prev_head = None;
    let mut new_head = head;

    loop {
        match head {
            None => {
                break;
            }
            Some(mut node) => {
                match node.next {
                    None => {
                        break;
                    }
                    Some(mut next) => {
                        let temp = next.next;
                        node.next = temp;
                        next.next = Some(node);
                        if new_head == None {
                            new_head = Some(next);
                        } else {
                            prev_head.next = Some(next);
                        }
                        let nextnext = head.next;
                        prev_head = head;
                    }
                }
                prev_head = head;
            }
        }
    }

    println!("{:?}", new_head);
}
