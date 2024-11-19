


// D D

// for j in 1..=5 {
//     if i >= j && vals[i] - vals[i - j] > 2 {
//         max_add = i64::max(max_add, dp[i - j]);
//     }
// }
//
// 直接每次往前找5个。



// long long maximumTotalDamage(vector<int>& power) {
//     long long dp[100001] = {0}, max_dp = 0;
//     sort(begin(power), end(power));
//     for (int i = 0, j = 0; i < power.size(); ++i)        
//         if (power[i] == power[max(0, i - 1)])
//             dp[i + 1] = power[i] + dp[i];
//         else {
//             while(power[j] + 2 < power[i])
//                 max_dp = max(max_dp, dp[++j]);
//             dp[i + 1] = power[i] + max_dp;
//         }
//     return *max_element(begin(dp), begin(dp) + power.size() + 1);
// }
// 
// 只要一次 for 循环？？？
// 好像确实。。。





// Runtime
// 67ms
// Beats67.31%
// Analyze Complexity
// Memory
// 5.16MB
// Beats75.00%


// -2, -1, +1, +2
// 1 1 4
// 1 6 6
impl Solution {

    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort();
        let mut vi = Vec::new();
        let mut vpower = Vec::new();

        vi.push(power[0] as i64);
        vpower.push(power[0]);
        for i in 1..power.len() {
            if power[i] == power[i - 1] {
                let len1 = vi.len() - 1;
                vi[len1] += power[i] as i64;
            } else {
                vi.push(power[i] as i64);
                vpower.push(power[i]);
            }
        }

        let mut vpfx = vec![0i64; vi.len()];

        use std::collections::VecDeque;

        let mut que : VecDeque<(i32, i64)> = VecDeque::new();

        let mut pfxmx = 0i64;

        for i in 0..vi.len() {
            while !que.is_empty() && que.front().unwrap().0 < vpower[i] - 2 {
                pfxmx = pfxmx.max(que.front().unwrap().1);
                que.pop_front();
            }

            vpfx[i] = pfxmx + vi[i];

            if i > 0 {
                vpfx[i] = vpfx[i].max(vpfx[i - 1]);
            }

            que.push_back((vpower[i], vpfx[i]));
        }

        // println!("{:?}\n{:?}", vi, vpower);
        // println!("{:?}", vpfx);

        let mut ans = vpfx[vpfx.len() - 1];

        que.clear();

        let mut t2 = 0i64;
        let mut idx = vpfx.len() as i32 - 1;
        let mut sfxmx = 0i64;
        for i in (0..vi.len()).rev() {

            while !que.is_empty() && que.front().unwrap().0 > vpower[i] + 2 {
                sfxmx = sfxmx.max(que.front().unwrap().1);
                que.pop_front();
            }

            t2 = t2.max(sfxmx + vi[i]);
            while idx >= 0 && vpower[idx as usize] >= vpower[i] - 2 {
                idx -= 1;
            }
            if idx >= 0 {
                ans = ans.max(t2 + vpfx[idx as usize]);
            } else {
                ans = ans.max(t2);
            }
            que.push_back((vpower[i], t2));
        }
        ans
    }


    // error . pfx1 pfx2 pfx3 pfx4
    pub fn maximum_total_damage_error(mut power: Vec<i32>) -> i64 {
        power.sort();
        let mut vi = Vec::new();
        let mut vpower = Vec::new();

        vi.push(power[0] as i64);
        vpower.push(power[0]);
        for i in 1..power.len() {
            if power[i] == power[i - 1] {
                let len1 = vi.len() - 1;
                vi[len1] += power[i] as i64;
            } else {
                vi.push(power[i] as i64);
                vpower.push(power[i]);
            }
        }

        let mut vpfx = vec![0i64; vi.len()];

        let mut pfx1 = 0i64; // vi[i] - 1 's max power
        let mut val1 = i32::MAX; // pfx1  <->  value
        let mut pfx2 = 0i64; // vi[i] - 2
        // let mut val2 = -1i32;
        let mut pfx3 = 0i64; // <= vi[i] - 3, max got

        for i in 0..vi.len() {
            // if val2 != -1 {
            //     val2 = -1;
            //     pfx3 = pfx3.max(pfx2);
            // }
            pfx3 = pfx3.max(pfx2);
            if val1 < vpower[i] - 2 {
                pfx3 = pfx3.max(pfx1);
            } else {
                // val2 = val1;
                pfx2 = pfx1;
            }
            vpfx[i] = pfx3 as i64 + vi[i];
            if i > 0 {
                vpfx[i] = vpfx[i].max(vpfx[i - 1]);
            }
            pfx1 = vpfx[i];
            val1 = vpower[i];
        }

        println!("{:?}\n{:?}", vi, vpower);
        println!("{:?}", vpfx);

        let mut ans = vpfx[vpfx.len() - 1];
        val1 = -1;
        pfx3 = 0; // suffix 3
        pfx2 = 0;
        pfx1 = 0;
        let mut t2 = 0i64;
        let mut idx = vpfx.len() as i32 - 1;
        for i in (0..vi.len()).rev() {
            println!("r {:?}", i);
            pfx3 = pfx3.max(pfx2);
            if val1 > vpower[i] + 2 {
                pfx3 = pfx3.max(pfx1);
            } else {
                pfx2 = pfx1;
            }
            t2 = t2.max(pfx3 as i64 + vi[i]);
            // if i == 0 {
            //     ans = ans.max(t2);
            // } else {
                while idx >= 0 && vpower[idx as usize] >= vpower[i] - 2 {
                    idx -= 1;
                }
                if idx >= 0 {
                    ans = ans.max(t2 + vpfx[idx as usize]);
                } else {
                    ans = ans.max(t2);
                }
            // }
            println!("  {}, {}, {}", ans, t2, pfx3);
            pfx1 = t2;
            val1 = vpower[i];
        }
        ans
    }
}



struct Solution {}

fn main()
{
    // let vi = [1,1,3,4].to_vec();

    let vi = [5,9,2,10,2,7,10,9,3,8].to_vec(); // 31


    println!("ans: {:?}", Solution::maximum_total_damage(vi));
}


