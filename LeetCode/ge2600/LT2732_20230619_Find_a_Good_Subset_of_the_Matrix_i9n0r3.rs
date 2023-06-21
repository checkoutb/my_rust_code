

// Runtime48 ms
// Beats
// 72.73%
// Memory3 MB
// Beats
// 72.73%

// hint1 : if exists, always 1 or 2

// sz1 < 10^4 , sz2 <= 5
// 0 or 1
// cnt1 <= cnt0 + 1
impl Solution {
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vi = vec![-1; 32];
        for i in 0..grid.len() {
            let mut t2: usize = 0;
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    t2 |= 1 << j;
                }
            }
            vi[t2] = i as i32;
        }
        let mut ans = Vec::new();
        if vi[0] != -1 {
            ans.push(vi[0]);
            return ans;
        }

        // println!("{:?}", vi);

        for i in 1..32 {
            if vi[i] == -1 {
                continue;
            }
            for j in (i + 1)..32 {
                if vi[j] != -1 && 0 == (i & j) {
                    ans.push(vi[i]);
                    ans.push(vi[j]);
                    ans.sort();
                    return ans;
                }
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    let vvi = [[0,1,1,0].to_vec(),[0,0,0,1].to_vec(),[1,1,1,1].to_vec()].to_vec();

    println!("ans: {:?}", Solution::good_subsetof_binary_matrix(vvi));
}


