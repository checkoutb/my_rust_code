







// Runtime
// 37ms
// Beats93.75%of users with Rust
// Memory
// 14.68MB
// Beats75.00%of users with Rust

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let sz1 = n as usize;
        let mut uf : Vec<(i32, i32)> = vec![(-1, i32::MAX); sz1]; //Vec::new();      // (uf root, &)
        for i in 0..edges.len() {
            // let (u, v, w) = edges[i];
            let u = edges[i][0];
            let v = edges[i][1];
            let w = edges[i][2];
            let ur = Self::ufa(&mut uf, u as usize);
            let vr = Self::ufa(&mut uf, v as usize);
            if ur != vr {
                uf[vr].0 = ur as i32;
                uf[ur].1 = w & uf[vr].1 & uf[ur].1;
            } else {
                uf[vr].1 = uf[vr].1 & w;
            }
        }

        let mut ans = vec![-123; query.len()];
        for i in 0..query.len() {
            // let (s, t) = query[i];
            let s = query[i][0];
            let t = query[i][1];
            let sr = Self::ufa(&mut uf, s as usize);
            let tr = Self::ufa(&mut uf, t as usize);
            if (sr != tr) {
                ans[i] = -1;
            } else {
                ans[i] = uf[sr].1;
            }
        }
        ans
    }

    fn ufa(uf: &mut Vec<(i32, i32)>, idx: usize) -> usize {
        if uf[idx].0 == -1 {
            idx
        } else {
            uf[idx].0 = Self::ufa(uf, uf[idx].0 as usize) as i32;
            uf[idx].0 as usize
        }
    }
}

struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


