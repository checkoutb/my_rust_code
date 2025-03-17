

// D D

// 分为2种， 以 A[i][0] 开始， 以A[i][1]结束。


// debug。。。。

// Runtime
// 73ms
// Beats16.67%
// Memory
// 11.10MB
// Beats59.26%



// [[30,49,12]]  28

impl Solution {
    pub fn maximum_coins(coins: Vec<Vec<i32>>, k: i32) -> i64 {
        
        let mut coins = coins;
        coins.sort();

        let mut ans = 0;
        let mut sum2 = 0;
        let mut idx = 0;  // coins end
        let mut en = 0;  // window's last

        let mut vpfx = vec![0; coins.len()];
        for i in 0..coins.len() {
            vpfx[i] = (coins[i][1] - coins[i][0] + 1) as i64 * coins[i][2] as i64;
            if i > 0 {
                vpfx[i] += vpfx[i - 1];
            }
        }


        for i in 0..coins.len() {
            println!("{:?}", coins[i]);
        }

        println!("pfx: {:?}", vpfx);

        for i in 0..coins.len() {
            while idx < coins.len() && coins[idx][0] <= coins[i][0] + k - 1 {
                idx += 1;
            }
            // idx is next of end
            idx -= 1;  // end]

            // println!("   start to cal: {} {}", i, idx);

            sum2 = 0;
            let mut st = coins[i][0];
            let mut idx2 = idx;

            if idx == i {
                sum2 = (coins[i][1] - coins[i][0] + 1).min(k) as i64 * coins[i][2] as i64;
                println!(" {} {} {}", i, idx, sum2);
                ans = ans.max(sum2);
            } else {
                sum2 = vpfx[idx - 1] - if i == 0 {0} else {vpfx[i - 1]};
                sum2 += ((coins[i][0] + k - 1).min(coins[idx][1]) - coins[idx][0] + 1) as i64 * coins[idx][2] as i64;
                println!(" --- {} {} {}", i, idx, sum2);
                ans = ans.max(sum2);
            }

            // let mut idx2 = idx;
            // while idx2 < coins.len() && 

            while idx < coins.len() && coins[idx][1] <= coins[i][1] + k - 1 {
                sum2 = vpfx[idx] - vpfx[i];
                let st = coins[idx][1] - k + 1;

                if st <= coins[i][1] {
                    sum2 += (coins[i][1] - st + 1).min(coins[i][1] - coins[i][0] + 1) as i64 * coins[i][2] as i64;
                }

                println!("{} {} {} {}={}-{} -- {} {} {}", i, idx, sum2, vpfx[idx] - vpfx[i], vpfx[idx], vpfx[i], coins[i][1], st, coins[i][2]);

                ans = ans.max(sum2);

                idx += 1;
            }




            // if coins[i][2] >= coins[idx][2] {   // part end
            //     if idx == i {
            //         sum2 = (coins[i][1] - coins[i][0] + 1).min(k) as i64 * coins[i][2] as i64;
            //     } else {
            //         let t2 = idx - 1;
            //         sum2 = vpfx[t2] - if i == 0 { 0 } else { vpfx[i - 1] };
            //         sum2 += ((coins[i][0] + k).min(coins[idx][1]) - coins[idx][0] + 1) as i64 * coins[idx][2] as i64
            //     }
            // } else {  // all end, part [i]
            //     let st = coins[idx][1] - k + 1;
            //     if st > coins[i][1] {
            //         sum2 = 0;
            //     } else {
            //         sum2 = vpfx[idx] - vpfx[i];
            //         sum2 += (coins[i][1] - st + 1) as i64 * coins[i][2] as i64;
            //     }
            // }

            // println!(" sum2: {} ", sum2);

            ans = ans.max(sum2);
        }

        ans
    }
}



struct Solution {}

fn main()
{

    // let vvi = [[8,10,1].to_vec(),[1,3,2].to_vec(),[5,6,4].to_vec()].to_vec();
    // let k = 4;

    // let vvi = [[8,12,13].to_vec(),[29,32,2].to_vec(),[13,15,2].to_vec(),[40,41,18].to_vec(),[42,48,18].to_vec(),[33,36,11].to_vec(),[37,38,6].to_vec()].to_vec();
    // let k = 28;   // 226

    let vvi = [[20,27,18].to_vec(),[37,40,19].to_vec(),[11,14,18].to_vec(),[8,10,9].to_vec(),[28,32,14].to_vec(),[1,7,5].to_vec()].to_vec();
    let k = 32;  // 380

    println!("ans: {:?}", Solution::maximum_coins(vvi, k));
}


