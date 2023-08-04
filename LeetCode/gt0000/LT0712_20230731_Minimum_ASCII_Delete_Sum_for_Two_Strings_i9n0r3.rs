



// D D

// if(a[i-1] == b[j-1])
//     dp[i][j] = dp[i-1][j-1];
// else
//     dp[i][j] = min( dp[i-1][j] + a[i-1] ,
//                 dp[i][j-1] + b[j-1] );

// 。。。



// 很不真实，总感觉百万的计算会tle。但是才4ms。。
// 如果a=0,z=25 是什么？ 或者 a=10,z=35 呢？


// Runtime4 ms
// Beats
// 100%
// Memory2.7 MB
// Beats
// 26.67%

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let sz1 = s1.len();
        let sz2 = s2.len();
        let mut vvi = vec![vec![0; sz2]; sz1];
        for i in 0..sz1 {
            for j in 0..sz2 {
                if s1[i] == s2[j] {
                    if i == 0 || j == 0 {
                        vvi[i][j] = s1[i] as i32;
                    } else {
                        vvi[i][j] = s1[i] as i32 + vvi[i - 1][j - 1];
                    }
                } else {
                    if i == 0 && j == 0 {
                        vvi[i][j] = 0;
                    } else if i != 0 && j != 0 {
                        vvi[i][j] = vvi[i - 1][j].max(vvi[i][j - 1]);
                    } else if i == 0 {
                        vvi[i][j] = vvi[i][j - 1];
                    } else if j == 0 {
                        vvi[i][j] = vvi[i - 1][j];
                    }
                }
            }
        }
        let mut ans :i32 = 0;
        for i in 0..sz1 {
            ans += s1[i] as i32;
        }
        for j in 0..sz2 {
            ans += s2[j] as i32;
        }
        println!("{:?}, \n, {:?}", s1, s2);
        println!("{:?}", vvi);
        ans - vvi[sz1 - 1][sz2 - 1] * 2
    }
}



struct Solution {}

fn main()
{
    // let s1 = "sea".to_string();
    // let s2 = "eat".to_string();

    let s1 = "a".to_string();
    let s2 = "at".to_string();

    println!("ans: {:?}", Solution::minimum_delete_sum(s1, s2));
}


