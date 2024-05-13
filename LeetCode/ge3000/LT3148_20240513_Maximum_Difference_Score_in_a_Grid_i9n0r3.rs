

// To calculate the best score ending at a cell ck,
// just need to find out the minimum c1 on its top left area.
// To do this, we can define dp[i][j] as the minimum in A[i][j] and its top-left.
// dp[i][j] = min(A[i][j], dp[i-1][j], dp[i][j-1]))


// int res = -1e6, m = A.size(), n = A[0].size();
// for (int i = 0; i < m; i++) {
//     for (int j = 0; j < n; j++) {
//         int pre = min(i > 0 ? A[i - 1][j] : 1e6, j > 0 ? A[i][j - 1] : 1e6);
//         res = max(res, A[i][j] - pre);
//         A[i][j] = min(A[i][j], pre);
//     }
// }
// return res;



// Runtime
// 15ms
// Beats100.00%of users with Rust
// Memory
// 3.75MB
// Beats100.00%of users with Rust

// down , or right 
// -[][] + [][]  max ,   0
impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {

        let sz1 = grid.len();
        let sz2 = grid[0].len();

        let mut vi = vec![0; sz2];

        let mut i = sz2 as i32 - 2;
        vi[i as usize + 1] = grid[sz1 - 1][sz2 - 1];
        let mut ans = -1234567;
        while i >= 0 {
            vi[i as usize] = vi[i as usize].max(vi[i as usize + 1].max(grid[sz1 - 1][i as usize]));     // max(grid)
            ans = ans.max(vi[i as usize + 1] - grid[sz1 - 1][i as usize]);
            i -= 1;
        }

        i = sz1 as i32 - 2;
        while i >= 0 {
            let mut j = sz2 as i32 - 2;
            ans = ans.max(vi[j as usize + 1] - grid[i as usize][j as usize + 1]);   // grid[][j+1]
            vi[j as usize + 1] = vi[j as usize + 1].max(grid[i as usize][sz2 - 1]);
            while j >= 0 {
                ans = ans.max(vi[j as usize].max(vi[j as usize + 1]) - grid[i as usize][j as usize]);
                vi[j as usize] = vi[j as usize].max(vi[j as usize + 1].max(grid[i as usize][j as usize]));
                j -= 1;
            }

            i -= 1;
        }

        ans

        // for i in 0..

        // let mut vi = vec![0; sz1];  // -
        // let mut v2 = vec![0; sz2];  // |

        // let mut i = sz1 as i32 - 1;
        // while i >= 0 {
        //     let mut j = sz2 as i32 - 1;
        //     while j >= 0 {

        //     }
        // }

    }
}



struct Solution {}

fn main()
{

    //[[6,5,1],     // 1->9
    // [5,7,9],
    // [6,7,4],
    // [6,10,5]]       // 8

    // 9 8 1
    // 2 9 3    // 7    // 2->9

    println!("ans: {:?}", Solution::());
}


