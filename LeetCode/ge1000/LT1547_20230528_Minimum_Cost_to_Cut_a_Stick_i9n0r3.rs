


// let mut cuts = cuts;
// cuts.push(0);
// cuts.push(n);
// cuts.sort();
// let mut dp = vec![vec![0; cuts.len()]; cuts.len()];
// for i in (0..cuts.len()).rev() {
//     for j in i + 2..cuts.len() {
//         dp[i][j] = i32::max_value();
//         for k in i + 1..j {
//             dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + cuts[j] - cuts[i]);
//         }
//     }
// }
// dp[0][cuts.len() - 1]



// Runtime24 ms
// Beats
// 66.67%
// Memory2.2 MB
// Beats
// 66.67%

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        //cuts.sort();
        let mut arr = cuts.clone();
        arr.sort();

        let sz1 = cuts.len();
        let mut vvi = vec![vec![-1; sz1]; sz1];

        Self::dfs_memo(&arr, &mut vvi, 0, sz1 - 1, 0, n)
    }

    fn dfs_memo(cuts: &Vec<i32>, vvi: &mut Vec<Vec<i32>>, st: usize, en: usize, stk_st: i32, stk_en: i32) -> i32 {
        if st > en {
            return 0;
        }
        if vvi[st][en] != -1 {
            return vvi[st][en];
        }
        if st == en {
            vvi[st][en] = stk_en - stk_st;
            return vvi[st][en];
        }
        let mut mn = 100000000;
        for i in st..=en {
            // let t2 = Self::dfs_memo(cuts, vvi, st, i - 1, stk_st, cuts[i])
            //             + Self::dfs_memo(cuts, vvi, i + 1, en, cuts[i], stk_en);
            let mut t2 = Self::dfs_memo(cuts, vvi, i + 1, en, cuts[i], stk_en);
            if i > 0 {
                t2 += Self::dfs_memo(cuts, vvi, st, i - 1, stk_st, cuts[i]);
            }
            if t2 < mn {
                mn = t2;
            }
        }
        vvi[st][en] = mn + stk_en - stk_st;
        return vvi[st][en];
    }
}



struct Solution {}

fn main()
{
    let n = 9;
    let arr: Vec<i32> = [5,6,1,4,2].to_vec();

    println!("ans: {:?}", Solution::min_cost(n, arr));
}


