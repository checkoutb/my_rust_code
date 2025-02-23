


// D

// 直接加 no delete， 差值 和 
// dfs的时候不管 边的数量是不是小于k。。。 nth_element


// hint 和我想的一样，但是写不来了。。

// g 代码太复杂了。。

// 每个点最多 k个边。  remain‘s sum max

// 不能移除最大的    [5,4,4,1]  [4,1,1] [4,1,1]  把2个4移除就可以了
// 不能移除最小的。   [5,4,4,3] [4,3,3] [4,3,3]  移除2个4

// 没有思路。

// 找出 边>k 的点， 如果 孤立点，那么 减去 最小的边，直到 边<=k。
// 如果 连通。  要计算代价。  后缀遍历？ 返回 2个值，一个是删除 与父节点的边，一个是不删除

impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut sze = edges.len();
        let mut vvi = vec![vec![]; sze + 1];
        let mut vb = vec![false; sze + 1];
        let mut ans = 0i64;
        
        for i in 0..edges.len() {
            let u = edges[i][0];
            let v = edges[i][1];
            let w = edges[i][2];
            vvi[u as usize].push((v, w));
            vvi[v as usize].push((u, w));
            ans += w as i64;
        }
        
        for i in 0..vvi.len() {
            vvi[i].sort_by_key(|a| a.1);
            if vvi[i].len() <= k as usize {
                vb[i] = true;   // not visit it
            }
        }

        for i in 0..vvi.len() {
            // if vvi[i].len() <= k as usize {
            //     continue;
            // }
            if vb[i] {
                continue;
            }
            let pii = Self::dfsa1(&vvi, &vb, i, -1, k);
            // ans -= pii.0.min(pii.1);
            ans -= pii.1;
        }
        
        ans
    }

    // 如果删除 和parent的边，那么就是 取 最小的 多余 vvi[].len() - k - 1 个的
    // 如果不删除， 取最小的 多余的 vvi[node].len() - k 个
    fn dfsa1(vvi: &Vec<Vec<i32, i32>>, vb: &Vec<bool>, node: usize, parent: i32, k: i32) -> (i64, i64) {
        if vb[node] {
            return (0, 0);
        }
        vb[node] = true;
        let mut del = 0;
        let mut nodel = 0;
        let mut vp = vec![];
        let mut v2 = vec![]; // to <=k's node
        let mut ppw = -1;    // parent path weight

        // for nxt : vvi[node] {
        for i in 0..vvi[node].len() {
            let nxt =  vvi[node][i].0;
            let w = vvi[node][i].1;
            if vb[nxt] {
                if nxt as i32 == parent {
                    ppw = w
                } else {
                    v2.push(w);
                }
                continue;
            }

            // pii.0 永远>= pii.1 ?  delete >= not delete . delete 减去更多
            let pii = Self::dfsa1(vvi, vb, nxt, node as i32, k);
            vp.push(pii);
            // if delete edge (node, nxt)

            // if not
        }

        v2.sort();

        // 如果必须删除 呢？

        // not delete
        // (100, 1)  (10, 2)   1+10
        vp.sort_by_key(|a| a.0 - a.1)

        let mut v2i = 0;
        let mut vpi = 0;
        let mut cnt = vvi[node].len() as i32 - k; // 一共删除

        if cnt as usize >= vp.len() + v2.len() {

            while cnt > 0 {
                cnt -= 1;
                
                if v2i == v2.len() {
                    nodelete += vp[];

                } else if vpi == vp.len() {

                } else {

                }
            }

            // 还要把剩余的 vp 加进来。
        } else {
            // 不够删除，所以必须删除 path to parent
        }
        


        // delete
        if parent == -1 {
            del = nodel;
        } else {

        }

        (del, nodel)
    }

    // return (delete, not delete)   min delete
    // fn dfsa1(vvi: &Vec<Vec<(i32, i32)>>, vb: &Vec<bool>, node: i32, parent: i32, k: i32) -> (i64, i64) {
    //     if vvi[node as usize].len() <= k as usize {
    //         return (0, 0);
    //     }
    //     vb[node as usize] = true;

    //     let mut del = 0;
    //     let mut nodel = 0;

    //     for i in 0..vvi[node as usize].len() {
            
    //     }

    //     (del, nodel)
    // }
}


struct Solution {}

fn main()
{
    let vvi = [[0,1,4].to_vec(), [0,2,2].to_vec(), [2,3,12].to_vec(), [2,4,6].to_vec(), ].to_vec();
    let k = 2;

    println!("ans: {:?}", Solution::maximize_sum_of_weights(vvi, k));
}


