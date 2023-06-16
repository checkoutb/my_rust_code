








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



    // Runtime112 ms
    // Beats
    // 19.5%
    // Memory16.9 MB
    // Beats
    // 71.43%
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let (dep, cnt) = Self::dfsa2(&root);
        let mut vi: Vec<usize> = vec![0; cnt as usize + 1];   // total depth
        let mut v2: Vec<usize> = vec![0; cnt as usize + 1];   // level
        // let mut vvi: Vec<i32> = vec![Vec::new(); dep as usize + 1];
        let mut vvi: Vec<Vec<usize>> = vec![vec![0; 2]; dep as usize + 1];
        
        Self::dfsb1(&root, &mut vi, &mut v2, &mut vvi, 0);

        println!("{:?}", vi);
        println!("{:?}", v2);

        let mut ans: Vec<i32> = vec![0; queries.len()];
        for i in 0..queries.len() {
            let t2 = queries[i] as usize;
            // ans[i] = if vi[t2] == vvi[v2[t2]][0] { vvi[v2[t2]][1] as i32 } else { vvi[v2[t2]][0] as i32 }
            if vi[t2] == vvi[v2[t2]][0] {
                ans[i] = vvi[v2[t2]][1] as i32;
            } else {
                ans[i] = vvi[v2[t2]][0] as i32;
            }
            if ans[i] == 0 {
                ans[i] = v2[t2] as i32 - 1;
            }
        }
        ans
    }

    fn dfsb1(node: &Option<Rc<RefCell<TreeNode>>>, vi: &mut Vec<usize>, v2: &mut Vec<usize>, 
        vvi: &mut Vec<Vec<usize>>, dep: usize) -> usize {
        if let Some(nd) = node {
            let t2 = nd.borrow().val as usize;
            let lft = Self::dfsb1(&nd.borrow().left, vi, v2, vvi, dep + 1) + dep;
            let rht = Self::dfsb1(&nd.borrow().right, vi, v2, vvi, dep + 1) + dep;
            v2[t2] = dep;
            vi[t2] = lft.max(rht);
            if vi[t2] > vvi[dep][0] {
                vvi[dep][1] = vvi[dep][0];
                vvi[dep][0] = vi[t2];
            } else if vi[t2] > vvi[dep][1] {
                vvi[dep][1] = vi[t2];
            }
            return vi[t2] - dep + 1;
        }
        return 0;
    }




    pub fn tree_queries_____(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let (dep, cnt) = Self::dfsa2(&root);
        // let mut vi: Vec<i32> = vec![0; cnt];        // node -> depth/level
        // // let mut v2: Vec<i32> = vec![0; dep];        // level -> max xx
        // let mut vvi: Vec<Vec<i32>> = vec![Vec::new(); dep]; // level -> nodes

        let mut vi: Vec<i32> = vec![0; cnt as usize + 1];            // node: depth
        let mut vvi: Vec<Vec<i32>> = vec![vec![0; 2]; cnt as usize + 1];     // node : depth
        let mut vv2: Vec<Vec<i32>> = vec![vec![0; 4]; dep as usize];     // depth: 3/4 max depth
        let mut root = root;


        // println!("{}", dep);
        // println!("{}", vv2[0].len());

        Self::dfsa1(&mut root, &mut vi, &mut vvi, &mut vv2, 0);
        
        // for v in &vvi {
        //     println!("{:?}", v);
        // }
        // for v in &vv2 {
        //     println!("{:?}", v);
        // }

        let mut ans: Vec<i32> = vec![0; queries.len()];
        for i in 0..queries.len() {
            ans[i] = Self::cala1(&vvi[queries[i] as usize], &vv2[vi[queries[i] as usize] as usize]);
            if ans[i] == 0 {
                ans[i] = vi[queries[i] as usize - 1];
            }
        }
        ans
    }

    fn cala1(node: &Vec<i32>, lvl: &Vec<i32>) -> i32 {
        if node[0] != lvl[0] {
            return lvl[0];
        } else if node[1] != lvl[1] {
            return lvl[1];
        }
        return lvl[2];
    }

    // level max, 3 number
    // fn dfsa1(node: &Option<Rc<RefCell<TreeNode>>>, vi: &mut Vec<i32>, 
    //                     vvi: &mut Vec<Vec<i32>>, dep: usize) -> usize {
    fn dfsa1(node: &Option<Rc<RefCell<TreeNode>>>, vi: &mut Vec<i32>, vvi: &mut Vec<Vec<i32>>, 
                vv2: &mut Vec<Vec<i32>>, dep: i32) -> i32 {

        if let Some(nd) = node {
            let t2 = nd.borrow().val as usize;
            // let lft = Self::dfsa1(&Rc::unwrap_or_clone(nd).take().left, vi, vvi, vv2, dep + 1) + dep;
            // let rht = Self::dfsa1(&Rc::unwrap_or_clone(nd).take().right, vi, vvi, vv2, dep + 1) + dep;
            // let lft = Self::dfsa1(&nd.into_inner().left, vi, vvi, vv2, dep + 1) + dep;
            // let rht = Self::dfsa1(&nd.into_inner().right, vi, vvi, vv2, dep + 1) + dep;
            let lft = Self::dfsa1(&nd.borrow().left, vi, vvi, vv2, dep + 1) + dep;
            let rht = Self::dfsa1(&nd.borrow().right, vi, vvi, vv2, dep + 1) + dep;
            vi[t2] = dep;
            if lft > rht {
                vvi[t2][0] = lft;
                vvi[t2][1] = rht;
            } else {
                vvi[t2][0] = rht;
                vvi[t2][1] = lft;
            }
            let mut idx: i32 = 2;
            while idx >= 0 && vv2[dep as usize][idx as usize] < lft {
                vv2[dep as usize][idx as usize + 1] = vv2[dep as usize][idx as usize];
                vv2[dep as usize][idx as usize] = lft;
                idx -= 1;
            }
            idx = 2;
            while idx >= 0 && vv2[dep as usize][idx as usize] < rht {
                vv2[dep as usize][idx as usize + 1] = vv2[dep as usize][idx as usize];
                vv2[dep as usize][idx as usize] = rht;
                idx -= 1;
            }
            return lft.max(rht) - dep + 1;
        }
        // match node {
        //     Some(nd) => ,
        //     None => return,
        // }
        return 0
    }

    // depth, count
    fn dfsa2(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            // Some(nd) => 1 + dfsa2(&Rc::unwrap_or_clone(nd).take().left) + dfsa2(&Rc::unwrap_or_clone(nd).take().right),
            Some(nd) => {
                // let lft = Self::dfsa2(&Rc::unwrap_or_clone(nd).take().left);
                // let rht = Self::dfsa2(&Rc::unwrap_or_clone(nd).take().right);
                let lft = Self::dfsa2(&nd.borrow().left);
                let rht = Self::dfsa2(&nd.borrow().right);
                (lft.0.max(rht.0) + 1, 1 + lft.1 + rht.1)
            },
            None => (0, 0),
        }
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

    let vi = [2,3,4].to_vec();

    println!("ans: {:?}", Solution::tree_queries(n5, vi));
}


