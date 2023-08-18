





// g daoxinpomie


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
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p1 = Some(Box::new(ListNode::new(-1)));
        let mut p2 = Some(Box::new(ListNode::new(-2)));
        let mut p11 = p1;
        let mut p22 = p2;
        let mut head = head;

        while head.is_some() {
            let t2 = head.unwrap();
            
            if t2.val >= x {
                p22.as_mut().unwrap().next = Some(t2.clone());
                p22 = p22.unwrap().next;
            } else {
                p11.as_mut().unwrap().next = Some(t2.clone());
                p11 = p11.unwrap().next;
            }
            head = t2.next;
        }
        p22.as_mut().unwrap().next = None;
        if p2.as_mut().unwrap().next.is_some() {
            p11.as_mut().unwrap().next = p2.as_mut().unwrap().next;
        }
        return p11.unwrap().next;


        // let p1 = Some(Box::new(ListNode::new(-1)));
        // let p2 = Some(Box::new(ListNode::new(-2)));
        // let mut p11 = p1;
        // let mut p22 = p2;

        // while head.is_some() {
        //     if head.borrow().val >= x {
        //         p22.borrow().next = head;
        //         p22 = p22.borrow().next;
        //     } else {
        //         p11.borrow().next = head;
        //         p11 = p11.borrow().next;
        //     }
        //     head = head.borrow().next();
        // }
        // p22.borrow().next = None;
        // if p2.borrow().next.is_some() {
        //     p11.borrow().next = p2.borrow().next;
        // }
        // return p11.borrow().next;


        // let p1 = Some(Box::new(ListNode::new(-1)));
        // let p2 = Some(Box::new(ListNode::new(-2)));
        // let mut p11 = p1;
        // let mut p22 = p2;
        // while head.as_ref().is_some() {
        //     if head.as_ref().unwrap().val >= x {
        //         p22.unwrap().next = head;
        //         // p22 = head;
        //         p22 = p22.unwrap().next;
        //     } else {
        //         p11.unwrap().next = head;
        //         // p11 = head;
        //         p11 = p11.unwrap().next;
        //     }
        //     head = head.unwrap().next;
        // }
        // p22.unwrap().next = None;
        // if p2.unwrap().next.is_some() {
        //     p11.unwrap().next = p2.unwrap().next;
        // }
        // return p11.unwrap().next;

        
        // if head.is_none() {
        //     return head;
        // }

        // let mut Box<ListNode> tail = head;
        // while head.is_some() {
        //     tail = tail.unwrap();
        // }
        // if tail.is_none() {
        //     return head;
        // }
        // let en = tail;
        // let mut st = head;
        // while st != en {
        //     if st.unwrap().val >= x {
                
        //     } else {

        //     }
        // }
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::partition(None, 32));
}


