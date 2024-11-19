







// ..看最后的代码，上升的不多，但是 option<rc<refcell>> 真的 很唬人啊。
// 。。主要是 rc可以自动穿透，刚开始 一路解包， rc -> refcell -> ref , ref无法处理了。。 看了以前的treenode的代码。。
// 话说，实现了 什么接口，就可以自动穿透？   std::ops::Deref


// Runtime
// 11ms
// Beats20.00%
// Analyze Complexity
// Memory
// 2.43MB
// Beats65.63%



// rust 使得这道题目的难度上升了两点点



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
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut vi = vec![];
        Self::dfsa1(&root, &mut vi);
        vi.sort();
        vi.reverse();
        return if vi.len() >= k as usize { vi[(k - 1) as usize] } else { -1 };
    }

    // (is perfect tree, tree's node count)   or  return i32, -1 == not perfact, >0 == tree's node count
    fn dfsa1(node: &Option<Rc<RefCell<TreeNode>>>, vi: &mut Vec<i32>) -> (bool, i32) {
        let mut is_perfect: bool = true;
        let mut node_cnt = 0;
        match node {
            Some(rc) => {
                let node2 = rc.borrow();
                let (la, lb) = Self::dfsa1(&node2.left, vi);
                let (ra, rb) = Self::dfsa1(&node2.right, vi);
                if la && ra && lb == rb {
                    // is_perfect = true;
                    node_cnt = lb + 1 + rb;
                    vi.push(node_cnt);
                } else {
                    is_perfect = false;
                }
            },
            None => (),
        }
        
        (is_perfect, node_cnt)
    }
}



struct Solution {}

fn main()
{

    // ok
    // let mut node2 = TreeNode::new(5);
    // node2.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    // let node = Some(Rc::new(RefCell::new(node2)));

    // ok
    let mut node = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    node.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    node.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));


    println!("ans: {:?}", Solution::kth_largest_perfect_subtree(node, 1));
}


