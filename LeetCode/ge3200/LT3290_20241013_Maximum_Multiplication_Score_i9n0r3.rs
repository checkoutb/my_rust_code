


// D D

// for i in 1..5 {
//     for j in i..=n {
//         dp[i][j] = std::cmp::max(
//             dp[i][j - 1],
//             dp[i - 1][j - 1] + a[i - 1] as i64 * b[j - 1] as i64
//         );
//     }
// }


// vector<long long> t(4, INT_MIN);
// for (long long n : b) {
//     t[3] = max(t[3], t[2] + n * a[3]);
//     t[2] = max(t[2], t[1] + n * a[2]);
//     t[1] = max(t[1], t[0] + n * a[1]);
//     t[0] = max(t[0], n * a[0]);
// }
// return t[3];


// Runtime
// 23ms
// Beats72.73%
// Analyze Complexity
// Memory
// 4.58MB
// Beats71.43%


// 10^10  一百亿。

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let szb = b.len();
        let mut vi = vec![0i64; szb];  // <=i  max ele

        for i in 0..szb {
            vi[i] = (a[0] as i64 * b[i] as i64) as i64;
            if i > 0 {
                vi[i] = vi[i].max(vi[i - 1]);
            }
        }

        for i in 1..a.len() {
            let mut v2 = vec![0i64; szb];

            for j in i..szb {
                v2[j] = vi[j - 1] + (a[i] as i64 * b[j] as i64) as i64;
                if j > i {
                    v2[j] = v2[j].max(v2[j - 1]);
                }
            }

            vi = v2;
        }

        vi[vi.len() - 1]
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


