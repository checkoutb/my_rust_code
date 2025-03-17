





// Runtime
// 33ms
// Beats81.82%
// Memory
// 18.36MB
// Beats68.18%



impl Solution {
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        
        let mut vvi = vec![vec![]; nums.len()];
        let mut vlen = vec![(-1, -1); 50001];   // length to next node's  value and level

        for i in 0..edges.len() {
            vvi[edges[i][0] as usize].push((edges[i][1], edges[i][2]));
            vvi[edges[i][1] as usize].push((edges[i][0], edges[i][2]));
        }

        let (a, b) = Self::dfsa1(&vvi, 0, -1, 0, &nums, 0, 0, &mut vlen, 0);

        let mut ans = vec![0; 2];
        ans[0] = a;
        ans[1] = b;
        ans
    }

    // length, count
    fn dfsa1(vvi: &Vec<Vec<(i32, i32)>>, node: i32, parent: i32, lvl: i32, nums: &Vec<i32>, stlen: i32, mut stlvl: i32, vlen: &mut Vec<(i32, i32)>, length: i32) -> (i32, i32) {
        let mut stlen = stlen;

        let node = node as usize;
        let orivlen = vlen[nums[node] as usize];
        if vlen[nums[node] as usize].0 != -1 {
            let t2 = vlen[nums[node] as usize].0;
            if t2 > stlen {
                stlen = t2;
                stlvl = vlen[nums[node] as usize].1;
            }
        }
        let vlenidx = nums[node] as usize;

        vlen[nums[node] as usize].1 = lvl + 1;

        let mut ans = (length - stlen, lvl - stlvl + 1);
        for i in 0..vvi[node].len() {
            if vvi[node][i].0 == parent {
                continue;
            }
            vlen[vlenidx].0 = length + vvi[node][i].1;

            let t2 = Self::dfsa1(vvi, vvi[node][i].0, node as i32, lvl + 1, nums, stlen, stlvl, vlen, length + vvi[node][i].1);

            if t2.0 > ans.0 || (t2.0 == ans.0 && t2.1 < ans.1) {
                ans = t2;
            }
        }

        vlen[nums[node] as usize] = orivlen;

        // println!(" {:?}, {} , {}", ans, node, lvl);

        ans
    }
}



struct Solution {}

fn main()
{

    let vvi = [[0,1,2].to_vec(),[1,2,3].to_vec(),[1,3,5].to_vec(),[1,4,4].to_vec(),[2,5,6].to_vec()].to_vec();
    let vi = [2,1,2,1,3,1].to_vec();

    println!("ans: {:?}", Solution::longest_special_path(vvi, vi));
}


