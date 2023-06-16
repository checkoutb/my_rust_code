





// Runtime29 ms
// Beats
// 33.33%
// Memory3.1 MB
// Beats
// 33.33%

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let mut lvl = 1;
        // let mut mx = root.unwrap().borrow().val;
        let mut lvl = -1;
        let mut mx = -2100000000;
        let mut vp = vec![root; 1];
        let mut t2 = 0;
        while !vp.is_empty() {
            let mut vp2 = Vec::new();
            t2 += 1;
            let mut t3 = 0;
            let mut t4 = 0;
            for rf in vp {
                if let Some(nd) = rf {
                    t4 = 1;
                    t3 += nd.borrow().val;
                    vp2.push((nd.borrow().left).clone());
                    vp2.push((nd.borrow().right).clone());
                }
            }
            if t3 > mx && t4 == 1 {
                mx = t3;
                lvl = t2;
            }
            vp = vp2;
        }
        lvl
    }
}




struct Solution {}


fn wtf(val : i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = TreeNode::new(val);
    let rfc = RefCell::new(node);
    let rc = Rc::new(rfc);
    let op = Some(rc);
    op
}

fn main()
{
    let mut n5 = wtf(1);
    let mut n8 = wtf(2);
    let n9 = wtf(3);
    let n2 = wtf(4);
    n8.as_mut().unwrap().borrow_mut().left = n2;
    n5.as_mut().unwrap().borrow_mut().left = n8;
    n5.as_mut().unwrap().borrow_mut().right = n9;

    println!("ans: {:?}", Solution::max_level_sum(n5));
}


