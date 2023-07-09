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


fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut lk1 = l1;
    let mut lk2 = l2;


    while let (
        Some(val_01),
        Some(val_02),
    ) = (lk1.as_ref().and_then(|item| Some(item.val)), lk2.as_ref().and_then(|item| Some(item.val))) {
        println!("{:?},{:?}", val_01, val_02);

        lk1 = lk1.and_then(|it| it.next);
        lk2 = lk2.and_then(|it| it.next);
    }
    
    Some(Box::new(ListNode::new(10)))
}

fn main() {
    let mut l1 = ListNode::new(10);

    l1.next = Some(Box::new(ListNode::new(11)));

    let mut l2 = ListNode::new(15);

    l2.next = Some(Box::new(ListNode::new(16)));

    add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
}