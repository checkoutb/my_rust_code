



        // for i in (0..len).rev() {
        //     for j in i + 1..len {
        //         for op in 0..=k {
        //             dp[i][j][op] = dp[i][j][op].max(dp[i + 1][j][op].max(dp[i][j - 1][op]));
        //             let cost = calc_cost(s[i], s[j]);
        //             if cost <= op {
        //                 dp[i][j][op] = dp[i][j][op].max(dp[i + 1][j - 1][op - cost] + 2);
        //             }
        //         }
        //     }
        // }
        // dp[0][len - 1][k]




// Runtime
// 106ms
// Beats86.96%
// Memory
// 35.22MB
// Beats43.48%




impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let sz1 = s.len();
        // let mut vvi = vec![vec![0; k as usize + 1]; sz1];
        let mut vvvi = vec![vec![vec![0; k as usize + 1]; sz1]; sz1];
        let s = s.into_bytes();
        
        for i in 0..sz1 {
            vvvi[i][i][0] = 1;
        }

        let mut ans = 1;
        for len2 in 1..sz1 {
            for i in 0 .. sz1 {
                let j = i + len2;
                if j >= sz1 {
                    break;
                }
                let t2 = Self::mndiff(s[i], s[j]);
                for a in 0..=k as usize {
                    let mut t3 = 0;
                    // i+1  j-1.    i j-1      i+1 j
                    if a >= t2 as usize {
                        t3 = 2 + vvvi[i + 1][j - 1][a - t2 as usize];
                    }
                    t3 = t3.max(vvvi[i + 1][j][a]);
                    t3 = t3.max(vvvi[i][j - 1][a]);
                    vvvi[i][j][a] = t3;
                    ans = ans.max(t3);
                }
            }
        }

        // println!("{:?}", vvvi);
        // for i in 0..sz1 {
        //     println!("{:?}", vvvi[i]);
        // }
        
        ans
    }

    fn mndiff(a: u8, b: u8) -> i32{
        let x = (a - b'a') as i32;
        let y = (b - b'a') as i32;

        ((x + 26 - y) % 26).min((y + 26 - x) % 26)
    }
}


struct Solution {}

fn main()
{

    // let s = "abcde".to_string();
    // let k = 2;

    let s = "aaazzz".to_string();
    let k = 4;
    
    println!("ans: {:?}", Solution::longest_palindromic_subsequence(s, k));
}


