

// D D

// int n = grid.size(), m = grid[0].size();
// long pre = 1, suf = 1, mod = 12345;
// vector<vector<int>> A(n, vector<int>(m, 1));
// for (int i = 0; i < n; i++) {
//     for (int j = 0; j < m; j++) {
//         A[i][j] = pre * A[i][j] % mod;
//         A[n - i - 1][m - j - 1] = suf * A[n - i - 1][m - j - 1] % mod;
//         pre = pre * grid[i][j] % mod;
//         suf = suf * grid[n - i - 1][m - j - 1] % mod;
//     }
// }
// return A;


// ...
// 想象成一维数组， [a][b] 就是 [row<a || (row==a && col < b) 的所有元素] * [row>a || (row==a && col > b) 的所有元素]

// 贻笑于大方之家


// Runtime71 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory18.4 MB
// Beats
// 100%

// mod 12345
// 100000  *  316
// 文章本天成
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        let MOD = 12345 as i32;
        let mut vvi = vec![vec![0 as i32; sz2]; sz1];
        for j in 0..sz2 {
            vvi[0][j] = grid[0][j] % MOD;
            grid[sz1 - 1][j] = grid[sz1 - 1][j] % MOD;
        }
        for i in 1..sz1 {
            for j in 0..sz2 {
                vvi[i][j] = (grid[i][j] % MOD * vvi[i - 1][j]) % MOD;
            }
        }
        for i in 2..=sz1 {
            for j in 0..sz2 {
                grid[sz1 - i][j] = (grid[sz1 - i][j] % MOD * grid[sz1 - i + 1][j]) % MOD;
            }
        }
        let mut vi = vec![0; sz2];      // *** ___
        let mut v2 = vec![0; sz2];      // ___ ***
        vi[0] = grid[0][0];
        for j in 1..sz2 {   
            vi[j] = (vi[j - 1] * grid[0][j]) % MOD;
        }
        v2[sz2 - 1] = grid[0][sz2 - 1];
        for j in 2..=sz2 {
            v2[sz2 - j] = (v2[sz2 - j + 1] * grid[0][sz2 - j]) % MOD;
        }
        let mut ans = vec![vec![0; sz2]; sz1];
        let mut t2 = 0;

        // println!("{:?}, {:?}, {:?}, {:?}", vvi, grid, vi, v2);

        for j in 0..sz2 {
            t2 = 1;
            if j > 0 {
                t2 = (t2 * vi[j - 1]) % MOD;
            }
            if j + 1 < sz2 {
                t2 = (t2 * v2[j + 1]) % MOD;
            }
            // println!("{:?}", t2);
            for i in 0..sz1 {
                let mut t3 = t2;
                if i > 0 {
                    t3 = (t3 * vvi[i - 1][j]) % MOD;
                }
                if i + 1 < sz1 {
                    t3 = (t3 * grid[i + 1][j]) % MOD;
                }
                ans[i][j] = t3;
            }
        }
        ans
    }
}




struct Solution {}

fn main()
{

    let vvi = [[1,2].to_vec(), [3,4].to_vec()].to_vec();

    println!("ans: {:?}", Solution::construct_product_matrix(vvi));
}


