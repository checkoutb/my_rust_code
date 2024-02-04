






// Runtime29 ms
// Beats
// 16%
// Memory2.4 MB
// Beats
// 96%
// 
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut vvi = vec![Vec::new(); n as usize];
        let mut vb = vec![false; n as usize];     // be beated
        for vi in &edges {
            vb[vi[1] as usize] = true;
            vvi[vi[0] as usize].push(vi[1]);
        }

        for i in 0..n {
            if vb[i as usize] {
                continue;
            }
            let cnt = Self::beatCount(&vvi, i as usize);
            if cnt == n {
                return i;
            }
        }

        -1
    }

    fn beatCount(vvi: &Vec<Vec<i32>>, node: usize) -> i32 {
        let n = vvi.len();
        let mut vb = vec![false; n];
        Self::dfsa1(vvi, node, &mut vb);
        let mut cnt = 0;
        for i in 0..n {
            if vb[i] {
                cnt += 1;
            }
        }
        // println!("aaa {}", cnt);
        return cnt;
    }

    fn dfsa1(vvi: &Vec<Vec<i32>>, node: usize, vb: &mut Vec<bool>) {
        if vb[node] {
            return;
        }
        vb[node] = true;        // beat self, so if cnt == n { return i }
        for nxt in &vvi[node] {
            Self::dfsa1(vvi, *nxt as usize, vb);
        }
    }
}



struct Solution {}

fn main()
{

    let n = 3;
    let vvi = [[0,1].to_vec(), [1,2].to_vec()].to_vec();

    println!("ans: {:?}", Solution::find_champion(n, vvi));
}


