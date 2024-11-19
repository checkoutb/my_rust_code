


// D D

// 不需要vvi，直接遍历每个node，通过 parent[] 找到 最近的一个 相同char的节点，然后 记录： <child, parent>
// 处理所有的 <child, parent>， 就是 修改 parent[]。
// 计算每个子树的节点个数




// Runtime
// 91ms
// Beats71.43%
// Analyze Complexity
// Memory
// 18.20MB
// Beats14.29%


// 向上搜索 相同char 的节点， 本节点的父节点 改成 搜索到的节点。

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let s = s.as_bytes().to_vec();
        let mut ans = vec![0i32; parent.len()];
        let mut vp = vec![-1; 26];  // parent
        let mut vvi = vec![vec![]; parent.len()];

        for i in 1..parent.len() {
            vvi[parent[i] as usize].push(i as i32);
        }

        Self::dfsa1(0, &mut vvi, &mut vp, &s);

        Self::dfsa2(0, &vvi, &mut ans);

        ans
    }

    fn dfsa2(node: i32, vvi: &Vec<Vec<i32>>, vi: &mut Vec<i32>) -> i32 {
        let mut ans = 1;
        for i in 0..vvi[node as usize].len() {
            if vvi[node as usize][i] >= 0 {
                ans += Self::dfsa2(vvi[node as usize][i], vvi, vi);
            }
        }
        vi[node as usize] = ans;
        ans
    }

    fn dfsa1(node: i32, vvi: &mut Vec<Vec<i32>>, vp: &mut Vec<i32>, s: &Vec<u8>) -> bool {
        let au8 = b'a';
        let vpidx = (s[node as usize] - au8) as usize;
        let t2 = vp[vpidx];
        let mut ans = false; // remove node
        if t2 != -1 {
            ans = true;
            vvi[t2 as usize].push(node); // new parent node
        }
        vp[vpidx] = node;

        let chds = vvi[node as usize].len();
        for i in 0..chds {
            let t3 = Self::dfsa1(vvi[node as usize][i], vvi, vp, s);
            if t3 {
                vvi[node as usize][i] = -vvi[node as usize][i];
            }
        }

        vp[vpidx] = t2;
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [-1,0,0,1,1,1].to_vec();
    let s = "abaabc".to_string();

    println!("ans: {:?}", Solution::find_subtree_sizes(vi, s));
}


