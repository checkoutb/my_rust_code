

// D D

// impl Solution {
//     pub fn minimum_coins(mut arr: Vec<i32>) -> i32 {
//       for i in (0 .. (arr.len() - 1) / 2).rev() {
//         arr[i] += *arr[i + 1 .. 2 * i + 3].iter_mut().min().unwrap();
//       }
//       return arr[0];
//     }
//   }



// pub fn minimum_coins(prices: Vec<i32>) -> i32 {
//     let mut dp = vec![i32::MAX; prices.len() + 1];
//     let len = prices.len();
//     dp[0] = 0;
//     for (i, p) in (1..).zip(prices) {
//         let base = dp[i - 1];
//         for j in (i..).take(i + 1) {
//             if j <= len {
//                 dp[j] = dp[j].min(base + p);
//             } else {
//                 break;
//             }
//         }
//     }
//     dp[len]
// }



// let len = prices.len();
// let mut dp = vec![0; len + 1];
// for (i, &price) in prices.iter().enumerate().rev() {
//     let left = (len - 1).min(i + 1);
//     let right = len.min((i + 1) * 2);
//     dp[i] = price + *dp[left..=right].iter().min().unwrap_or(&0);
// }
// dp[0]



// (1..=n).for_each(|i| {
//     let sdp = dp[i - 1] + prices[i - 1];
//     (0..=i).for_each(|j| dp[n.min(i + j)] = sdp.min(dp[n.min(i + j)]));
// });



// Runtime0 ms
// Beats
// 100%
// Memory2.1 MB
// Beats
// 54.84%

impl Solution {


    // this && next i,  1-index
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let sz1 = prices.len();
        if sz1 == 1 {
            return prices[0];
        }
        // let MX = i32::MAX >> 2;
        let mut vi = vec![i32::MAX; sz1];
        vi[sz1 - 1] = prices[sz1 - 1];

        let mut idx = sz1 - 2;
        // let mut t2 = vi[sz1 - 1];
        // let mut fst = sz1 - 1;
        let mut t2 = 0usize;
        let mut t3 = 0i32;
        // while idx >= 0 {
        while true {        // idx == 0, break;
            // t2 = idx + idx + 1 + 1;         // end's next
            // if t2 >= sz1 {      // t3 = vi.get(t2).unwrap_or_default(0);
            //     t3 = 0;
            // } else {
            //     t3 = vi[t2];
            // }
            // t3 += prices[idx];
            // t3 = t3.min(vi[sz1 - 1]);
            
            t3 = i32::MAX;
            t2 = idx * 2 + 2;
            if t2 >= sz1 {
                t3 = 0;
            } else {
                for i in (idx + 1)..=t2 {
                    t3 = t3.min(vi[i]);
                }
            }
            vi[idx] = t3 + prices[idx];

            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        println!("{:?}", vi);

        vi[0]
    }
}



struct Solution {}

fn main()
{

    // let vi = [3,1,2].to_vec();
    let vi = [1,10,1,1].to_vec();

    println!("ans: {:?}", Solution::minimum_coins(vi));
}


