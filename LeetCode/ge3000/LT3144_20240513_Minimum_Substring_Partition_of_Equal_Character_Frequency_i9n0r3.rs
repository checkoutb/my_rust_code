


// vector<int> dp(s.size() + 1, s.size());
// dp.back() = 0;
// for (int i = s.size() - 1; i >= 0; --i) {
//     int cnt[26] = {}, unique = 0, max_cnt = 0;
//     for (int sz = 1; i + sz <= s.size(); ++sz) {
//         unique += ++cnt[s[i + sz - 1] - 'a'] == 1;
//         max_cnt = max(max_cnt, cnt[s[i + sz - 1] - 'a']);
//         if (sz == unique * max_cnt)
//             dp[i] = min(dp[i], 1 + dp[i + sz]);
//     }
// }
// return dp[0];
// ...... unique * max_cnt


// Runtime
// 60ms
// Beats100.00%of users with Rust
// Memory
// 2.35MB
// Beats100.00%of users with Rust

// same count
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut vvi = vec![vec![0; 26]; s.len()];
        let mut vi = vec![0; s.len()];
        vi[0] = 1;
        let mut sit = s.chars();
        let ausize = b'a' as usize;
        vvi[0][sit.next().unwrap() as usize - ausize] = 1;

        for i in 1..s.len() {
            for j in 0..26 {
                vvi[i][j] = vvi[i - 1][j];
            }
            vvi[i][sit.next().unwrap() as usize - ausize] += 1;
        }

        let mut b2;
        let mut t2;
        for i in 1..s.len() {
            b2 = true;
            t2 = -1;
            for k in 0..26 {
                if vvi[i][k] != 0 {
                    if t2 == -1 {
                        t2 = vvi[i][k];
                    } else if t2 != vvi[i][k] {
                        b2 = false;
                        break;
                    }
                }
            }
            if b2 {
                vi[i] = 1;
                continue;
            }

            vi[i] = i + 1;
            for j in 0..i {
                // if vi[j] != 0 {
                    b2 = true;
                    t2 = -1;
                    for k in 0..26 {
                        if vvi[i][k] - vvi[j][k] != 0 {     // i-j, not j-i ...
                            if t2 == -1 {
                                t2 = vvi[i][k] - vvi[j][k];
                            } else if t2 != vvi[i][k] - vvi[j][k] {
                                b2 = false;
                                break;
                            }
                        }
                    }
                    if b2 {
                        // vi[i] = vi[j] + 1;
                        vi[i] = vi[i].min(vi[j] + 1);
                        // println!("{}, {}, {}", i, j, t2);
                        // break;
                    }
                // }
            }
            // println!("{:?}", vi);
        }
        // println!("{:?}", vvi);
        vi[s.len() - 1] as i32
    }
}


struct Solution {}

fn main()
{

    // let s = "abababaccddb".to_string();
    let s = "aabcca".to_string();   // 3

    println!("ans: {:?}", Solution::minimum_substrings_in_partition(s));
}


