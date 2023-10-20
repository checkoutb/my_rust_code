




// Runtime6 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory3 MB
// Beats
// 100%

// 1 input, 2 output ...  root 0 input
// leftchild, rightchild, so exactly 2 output.
// so check 1 input ? only one(root) is 0 input             ... no...
impl Solution {


    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut vb = vec![false; n as usize];
        for i in 0..(n as usize) {
            if left_child[i] != -1 {

                if vb[left_child[i] as usize] {
                    return false;
                } else {
                    vb[left_child[i] as usize] = true;
                }
            }
            if right_child[i] != -1 {

                if vb[right_child[i] as usize] {
                    return false;
                } else {
                    vb[right_child[i] as usize] = true;
                }
            }
        }
        let mut root = n + 1;
        for i in 0..(n as usize) {
            if !vb[i] {
                if root == n + 1 {
                    root = i as i32;
                } else {
                    return false;
                }
            }
        }
        if root == n + 1 {
            return false;
        }
        return Self::dfsa2(&left_child, &right_child, root) == n;
    }

    fn dfsa2(lch: &Vec<i32>, rch: &Vec<i32>, node: i32) -> i32 {
        if node == -1 {
            return 0;
        }
        return 1 + Self::dfsa2(lch, rch, lch[node as usize]) + Self::dfsa2(lch, rch, rch[node as usize]);
    }

    // error.  1->0, 0->1  2->3 ...
    // pub fn validate_binary_tree_nodes__error(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    //     let mut vb: Vec<bool> = vec![false; n as usize];
    //     for i in 0..(n) {
    //         if !dfsa1(left_child, right_child, vb, i) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    // fn dfsa1(lch: Vec<i32>, rch: Vec<i32>, mut vb: Vec<bool>, node i32) -> bool {
    //     if node == -1 || vb[node as usize] {
    //         return true;
    //     }
    //     if dfsa1(lch, rch, vb, lch[node as usize]) && dfsa1(lch, rch, rch[node as usize]) {
    //         vb[node as usize] = true;
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }


    // 1->0, 0->1  2->3 ...
    pub fn validate_binary_tree_nodes___error(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut vb = vec![false; n as usize];
        for i in 0..(n as usize) {
            if left_child[i] != -1 {

                if vb[left_child[i] as usize] {
                    return false;
                } else {
                    vb[left_child[i] as usize] = true;
                }
            }
            if right_child[i] != -1 {

                if vb[right_child[i] as usize] {
                    return false;
                } else {
                    vb[right_child[i] as usize] = true;
                }
            }
            // if left_child[i] || right_child[i] {
            //     return false;
            // }
            // vb[left_child]
        }
        if vb.into_iter().filter(|x| !x).count() != 1 {
            return false;
        }
        return true;
    }
}






struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


