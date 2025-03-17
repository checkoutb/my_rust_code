

// D D

// bfs 可达


// Runtime
// 135ms
// Beats25.81%
// Memory
// 14.73MB
// Beats32.26%



// ？threshold 是不是没什么用？


impl Solution {
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        
        let mut ans = -1;

        let mut st = 1;
        let mut en = 1000000;

        let mut vvi = vec![vec![]; n as usize];

        for i in 0..edges.len() {
            vvi[edges[i][1] as usize].push((edges[i][0], edges[i][2]));
        }

        while st <= en {
            let md = (st + en) / 2;  // <= md
            let mut vst = vec![false; n as usize];
            let t2 = Self::dfs_a1(&vvi, md, 0, &mut vst);
            // println!("{} {}", md, t2);
            if t2 == n {
                ans = md;
                en = md - 1;
            } else {
                st = md + 1;
            }
        }

        ans
    }

    fn dfs_a1(vvi: &Vec<Vec<(i32, i32)>>, md: i32, node: usize, vst: &mut Vec<bool>) -> i32 {
        vst[node] = true;
        let mut ans = 1;
        for i in 0..vvi[node].len() {
            if vvi[node][i].1 > md {
                continue;
            }
            if vst[vvi[node][i].0 as usize] {
                continue;
            }
            ans += Self::dfs_a1(vvi, md, vvi[node][i].0 as usize, vst);
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let n = 5;
    let vvi = [[1,0,1].to_vec(),[2,0,2].to_vec(),[3,0,1].to_vec(),[4,3,1].to_vec(),[2,1,1].to_vec()].to_vec();
    let th = 2;


    println!("ans: {:?}", Solution::min_max_weight(n,vvi,th));
}


