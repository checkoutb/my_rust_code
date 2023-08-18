




// for coin in coins {
//     for curr_amount in coin..=amount {
//         dp[curr_amount as usize] += dp[(curr_amount - coin) as usize]
//     }
// }


// Runtime198 ms
// Beats
// 7.14%
// Memory6.5 MB
// Beats
// 14.29%
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        let amount = amount as usize;
        coins.sort();

        let mut vvi = vec![vec![0; coins.len()]; amount + 1];
        vvi[0][0] = 1;      // [amount][max]

        for i in 0..amount {
            for j in 0..coins.len() {
                for k in j..coins.len() {
                    if i + coins[k] as usize > amount {
                        break;
                    }
                    vvi[i + coins[k] as usize][k] += vvi[i][j];
                }
            }
        }
        let mut ans = 0;
        for j in 0..coins.len() {
            ans += vvi[amount][j];
        }
        ans

        // let mut coins = coins;
        // let mut vi = vec![0; amount + 1];
        // coins.sort();
        // vi[0] = 1;
        // for i in 0..amount {
        //     if vi[i] != 0 {
        //         for j in coins {
        //             if i + j > amount {
        //                 break;
        //             }

        //         }
        //     }
        // }
    }
}



struct Solution {}

fn main()
{
    let vi = [1,2,5].to_vec();
    let a = 5;

    println!("ans: {:?}", Solution::change(a, vi));
}


