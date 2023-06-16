






// Runtime0 ms
// Beats
// 100%
// Memory3.1 MB
// Beats
// 22.22%

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::dfsa1(&root, &mut ans, 0);
        ans
    }

    fn dfsa1(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>, dep: usize) {
        if let Some(nd) = node {
            if ans.len() == dep {
                ans.push(nd.borrow().val);
            } else {
                if nd.borrow().val > ans[dep] {
                    ans[dep] = nd.borrow().val;
                }
            }
            Self::dfsa1(&nd.borrow().left, ans, dep + 1);
            Self::dfsa1(&nd.borrow().right, ans, dep + 1);
        }
    }
}



struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


