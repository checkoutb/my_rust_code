





    // let mut dp: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    // let mut solution = 1;
    // for a in arr {
    //     dp.insert(a, dp.get(&(a - difference)).unwrap_or(&0) + 1);
    //     solution = solution.max(dp[&a]);
    // }
    // solution




// Runtime7 ms
// Beats
// 100%
// Memory3 MB
// Beats
// 95%

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut vi: Vec<i32> = vec![0; 20001];
        let mut ans: i32 = 1;
        let mut t2 = 0;
        for val in arr {
            let val = val + 10000;
            t2 = val - difference;
            if t2 >= 0 && t2 < (vi.len() as i32) {
                vi[val as usize] = (vi[t2 as usize] + 1).max(vi[val as usize]);
                if vi[val as usize] > ans {
                    ans = vi[val as usize];
                }
            } else {
                vi[val as usize] = 1;
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{
    // let vi = [1,5,7,8,5,3,4,2,1].to_vec();
    // let df = -2;

    let vi = [3,4,-3,-2,-4].to_vec();
    let df = -5;

    println!("ans: {:?}", Solution::longest_subsequence(vi, df));
}


