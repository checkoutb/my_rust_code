

// D D

// let len = std::iter::successors(head.as_ref(), |node| node.next.as_ref()).count();
// let mut dummy = ListNode { val: 0, next: head };
// let mut prev = (0..len - n as usize).fold(&mut dummy, |acc, _| acc.next.as_mut().unwrap());
// prev.next = prev.next.as_mut()?.next.take();
// dummy.next


// fn remove_nth_from_end_recr(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
//     match head {
//         None => (None, 1),
//         Some(mut node) => {
//             let (next, cnt) = Self::remove_nth_from_end_recr(node.next.take(), n);
//             if cnt == n {
//                 (next, cnt + 1)
//             } else {
//                 node.next = next;
//                 (Some(node), cnt + 1)
//             }
//         }
//     }
// }



// 看了Discuss，不然编译不可能成功。


// Runtime
// 1ms
// Beats76.43%of users with Rust
// Memory
// 2.02MB
// Beats71.97%of users with Rust



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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        
        let mut dummy = Box::new(ListNode::new(-12345));
        dummy.next = head;

        let mut fst = dummy.clone();
        let mut snd = dummy.as_mut();

        // fst.as_mut().next.as_mut().unwrap().val = 123;
        // snd.as_mut().next.as_mut().unwrap().val = 555;
        // println!("{} {} {}", fst.as_ref().next.as_ref().unwrap().val, snd.as_ref().next.as_ref().unwrap().val, dummy.as_ref().next.as_ref().unwrap().val);
        //   123 555 1

        for _ in 0..n {
            fst = fst.next.unwrap();
        }
        while fst.next.is_some() {
            fst = fst.next.unwrap();
            snd = snd.next.as_mut().unwrap();
        }

        // cannot move out of a mutable reference, need add ".clone()"
        snd.next = snd.next.as_mut().unwrap().next.clone();
        dummy.next


        // let mut fst = &mut dummy.clone();
        // for __i in 0..n {
        //     let fst2 = fst.as_mut().next.as_mut().unwrap();
        //     fst = fst2;
        // }
        // let mut snd = dummy.as_mut();
        // while fst.as_ref().next.is_some() {
        //     fst = fst.as_mut().next.as_mut().unwrap();
        //     snd = snd.next.clone().unwrap();
        // }
        // // snd.next = snd.as_ref().next.as_ref().unwrap().next;
        // snd.next = snd.next.as_ref().unwrap().next;
        // dummy.next
    }
}


struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


