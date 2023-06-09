
// D D

//dp[i] = dp[i].max(dp[i + 1]);


// use std::cmp::max;
// ans = max(ans, dp[i]);

// Runtime38 ms
// Beats
// 61.37%
// Memory9.4 MB
// Beats
// 46.80%
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let sz1: usize = questions.len();
        let mut vi: Vec<i64> = vec![0; sz1];
        let mut ans: i64 = 0;
        let mut mx: i64 = 0;
        for i in 0..sz1 {
            if vi[i] > mx {
                mx = vi[i];
            }
            let nxt: usize = i + questions[i][1] as usize + 1;
            let t2: i64 = mx + questions[i][0] as i64;
            if nxt >= sz1 {
                if t2 > ans {
                    ans = t2;
                }
            } else {
                if t2 > vi[nxt] {
                    vi[nxt] = t2;
                }
            }
            // if (questions[i][1] + i >= sz1 {
            //     if (mx + (questions[i][0] as i64) > ans) {
            //         ans = mx + (questions[i][0] as i64);
            //     }
            // } else {
            //     if (mx + (questions[i][0] as i64) > vi[])
            // }
        }
        ans
    }
}




struct Solution {}

fn main()
{

    let vvi = [[3,2].to_vec(),[4,3].to_vec(),[4,4].to_vec(),[2,5].to_vec()].to_vec();


    println!("ans: {:?}", Solution::most_points(vvi));
}


