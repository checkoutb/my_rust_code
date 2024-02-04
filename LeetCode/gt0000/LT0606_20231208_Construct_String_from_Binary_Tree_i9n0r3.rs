

// D D

// impl Solution {
//     pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         match root {
//             Some(node) => {
//                 let node_ref = node.borrow();
//                 let left = Solution::tree2str(node_ref.left.clone());
//                 let right = Solution::tree2str(node_ref.right.clone());
//                 if left == "" && right == "" {
//                     node_ref.val.to_string()
//                 } else if right == "" {
//                     format!("{}({})", node_ref.val, left)
//                 } else {
//                     format!("{}({})({})", node_ref.val, left, right)
//                 }
//             }
//             None => "".to_string()
//         }
//     }
// }


// pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//     let mut str = String::new();
//     if let Some(node) = root {
//         let mut nb = node.borrow_mut();
//         str.push_str(&nb.val.to_string());
//         if nb.left.is_some() || nb.right.is_some() {
//             str.push('(');
//             str.push_str(&Self::tree2str(nb.left.take()));
//             str.push(')');
//             if nb.right.is_some() {
//                 str.push('(');
//                 str.push_str(&Self::tree2str(nb.right.take()));
//                 str.push(')');
//             }
//         }
//     }
//     str
// }






// Runtime3 ms
// Beats
// 68.18%
// Memory2.6 MB
// Beats
// 77.27%


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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans: String = String::new();

        Self::preorder(&root, &mut ans, true);

        ans.pop();
        ans.remove(0);      // remove ( )
        ans
    }

    // return: is empty
    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut String, is_left: bool) -> bool {
        if let Some(nd) = node {
            ans.push('(');
            ans.push_str(nd.borrow().val.to_string().as_str());
            let lft = Self::preorder(&nd.borrow().left, ans, true);
            let rgh = Self::preorder(&nd.borrow().right, ans, false);
            if lft && rgh {
                ans.pop();
                ans.pop();      // remove the left child's "()"
            }
            ans.push(')');
            false
        } else {
            if is_left {
                ans.push_str("()");
            }
            true
        }

        // if node.is_none() {
        //     if is_left {
        //         ans.push_str("()");
        //     }
        //     return;
        // }

    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


