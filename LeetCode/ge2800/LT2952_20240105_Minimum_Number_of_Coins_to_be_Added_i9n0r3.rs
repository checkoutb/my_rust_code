
// D D

// while sum < target {
//     if i == coins.len() || sum < coins[i] - 1 {
//         sum += sum + 1;
//         ans += 1;
//     } else {
//         sum += coins[i];
//         i += 1;
//     }
// }



// Runtime25 ms
// Beats
// 22.50%
// Memory3.4 MB
// Beats
// 7.50%

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        // let mut vb = vec![false; (target + 1) as usize];
        // for i in 0..coins.len() {
        //     vb[coins[i] as usize] = true;
        // }
        
        let mut coins = coins;
        coins.sort();
        let mut got = 0;
        // let mut need = 1;
        let mut ans = 0;
        let mut idx = 0;

        while got < target {
            while idx < coins.len() && coins[idx] <= got + 1 && got < target {
                got += coins[idx];
                idx += 1;
            }
            if got >= target {
                break;
            }
            ans += 1;       // add (got + 1) to coins
            got += got + 1;
        }
        ans
    }
}



struct Solution {}

fn main()
{
    // let vi = [1,4,10].to_vec();
    // let tar = 19;

    let vi = [1,1,1].to_vec();
    let tar = 20;

    println!("ans: {:?}", Solution::minimum_added_coins(vi, tar));
}


