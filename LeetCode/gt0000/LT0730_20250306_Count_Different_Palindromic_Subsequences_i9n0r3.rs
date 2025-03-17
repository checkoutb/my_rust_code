










// g



// 

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let sz1 = s.len();

        const MOD: i32 = 1000000007;
        let mut vvi = vec![vec![0; sz1]; sz1];
        let s = s.into_bytes();
        for i in 0..sz1 {
            vvi[i][i] = 1;
        }
        let mut ans = 0;

        let mut vpw = vec![0; 501];   // 2^x
        let mut mxpw = 0;
        vpw[0] = 1;
        let mut powof2 = |pw| -> i32 {
            if pw > mxpw {
                for i in (mxpw + 1) as usize ..= (pw as usize) {
                    vpw[i] = vpw[i - 1] * 2 % MOD;
                }
                mxpw = pw;
            }
            vpw[pw]
        };
        
        // fn powof2(pw: i32) -> i32 {
        //     if pw > mxpw {
        //         for i in (mxpw + 1) as usize .. =pw as usize {
        //             vpw[i] = vpw[i - 1] * 2 % MOD;
        //         }
        //         mxpw = pw;
        //     }
        //     vpw[pw]
        // }
        
        for k in 1..sz1 {  // length - 1
            for i in 0..sz1 {
                let j = i + k;
                if j == sz1 {
                    break;
                }
                vvi[i][j] = vvi[i + 1][j].max(vvi[i][j - 1]);
                if s[i] == s[j] {
                    vvi[i][j] = vvi[i][j].max(1 + vvi[i + 1][j - 1]);

                    let t2 = vvi[i + 1][j - 1];
                    // 2^t2 ?
                    ans = (ans + powof2(t2)) % MOD;
                }
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let s = "bccb".to_string();
    

    println!("ans: {:?}", Solution::count_palindromic_subsequences(s));
}


