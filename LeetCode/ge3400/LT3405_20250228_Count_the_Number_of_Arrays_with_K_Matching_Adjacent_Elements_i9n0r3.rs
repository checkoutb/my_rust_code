





// tle


// 1-m, ==
// 就是 vi[0; k+1]. 但是会tle吗？

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        
        const MOD: i64 = 1000000007;

        let k1 = k as usize + 1;
        let mut vi = vec![0i64; k1];
        vi[0] = m as i64;

        let mut n = n;
        while n > 1 {
            n -= 1;
            let mut v2 = vec![0i64; k1];

            for i in 0..vi.len() {
                // if vi[i] == 0 {  // 3,1,2
                //     break;
                // }
                if i + 1 < vi.len() {
                    v2[i + 1] = (v2[i + 1] + vi[i]) % MOD;
                }
                v2[i] = (vi[i] * i64::from(m - 1) + v2[i]) % MOD;
            }
            vi = v2;
            // println!("{:?}", vi);  // Output Limit Exceeded
        }
        vi[k as usize] as i32
    }
}


struct Solution {}

fn main()
{

    let n = 3;
    let m = 2;
    let k = 1;


    println!("ans: {:?}", Solution::count_good_arrays(n,m,k));
}


