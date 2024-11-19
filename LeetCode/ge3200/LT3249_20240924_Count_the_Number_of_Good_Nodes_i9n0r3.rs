








// 写起来还是很。。C++本地不测，直接用leetcode的testcase测试。
// rust不行，本地跑了4,5次，就是改语法错误。  &, mut, *, as


// Runtime
// 115ms
// Beats96.77%
// Analyze Complexity
// Memory
// 20.28MB
// Beats100.00%



// 非二叉树， 所以 只有一个child时，必然good。
// 只需要个数相同，不需要同构


impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let sz1 = edges.len();
        let mut vvi = vec![Vec::new(); 0];
        vvi.resize(sz1 + 1, Vec::new());

        for vi in edges {
            vvi[vi[0] as usize].push(vi[1]);
            vvi[vi[1] as usize].push(vi[0]);
        }

        let mut ans = 0i32;

        Self::dfsa1(0, -1, &vvi, &mut ans);

        ans
    }

    // node count
    fn dfsa1(node: i32, parent: i32, vvi: &Vec<Vec<i32>>, ans: &mut i32) -> i32 {
        let mut cnt = 1i32;
        let mut t2 = -1i32;
        let mut ok = true;
        for nxt in &vvi[node as usize] {
            if *nxt == parent {
                continue;
            }
            let t3 = Self::dfsa1(*nxt, node, vvi, ans);
            cnt += t3;
            if t2 == -1i32 {
                t2 = t3;
            } else {
                if t2 != t3 {
                    ok = false;
                }
            }
        }

        if ok {
            *ans += 1;
        }

        cnt
    }
}




struct Solution {}

fn main()
{

    let vvi = [[0,1].to_vec(), [0,2].to_vec()].to_vec();

    println!("ans: {:?}", Solution::count_good_nodes(vvi));
}


