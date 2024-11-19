

// D D

// pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
//     const MAX: usize =100_001;
//     const MOD: i64 = 1e9 as i64 + 7;
//     let mut counts = [0; MAX];
//     let mut sum = [0; MAX];
//     for &num in &nums {
//         let num = i64::from(num);
//         let mut sum_num = num;
//         let mut cnt_num = 1;
//         for prev in [num - 1, num + 1] {
//             if prev < 0 || prev >= MAX as i64 {
//                 continue;
//             }
//             let cnt_prev = counts[prev as usize];
//             let sum_prev = sum[prev as usize];
//             sum_num = (sum_num + sum_prev + cnt_prev * num) % MOD;
//             cnt_num = (cnt_num + cnt_prev) % MOD;
//         }
//         counts[num as usize] = (counts[num as usize] + cnt_num) % MOD;
//         sum[num as usize] = (sum[num as usize] + sum_num) % MOD;
//     }
//     (sum.iter().sum::<i64>() % MOD) as i32
// }


// for (int a : A) {
//     count[a + 1] = (count[a] + count[a + 1] + count[a + 2] + 1) % mod;
//     long cur = total[a] + total[a + 2] + 1l * a * (count[a] + count[a + 2] + 1);
//     total[a + 1] = (total[a + 1] + cur) % mod;
//     res = (res + cur) % mod;
// }
// return (int)res;


// Runtime
// 331ms
// Beats6.38%
// Analyze Complexity
// Memory
// 4.64MB
// Beats53.19%



// tle， 应该是 clone的问题？


// 每个元素 可以作为一个 good subseq， 然后 再遍历一次，就变成 2个元素 的 good subseq。 但 O(n^2) 。
// arr[x] = y  以 x 结尾的 所有 good subseq 的 和。 外层还有一个 for循环。 还需要记录 subseq个数


impl Solution {
    pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        let mut vsum = vec![0i64; 100002];
        let mut vcnt = vec![0i64; 100002];
        const MOD: i64 = 1000000007;

        
        for i in 0..nums.len() {
            // let mut vsum2 = vec![0i64; vsum.len()];
            // let mut vcnt2 = vec![0i64; vcnt.len()];

            // let mut vsum2 = vsum.clone();
            // let mut vcnt2 = vcnt.clone();

            let t2 = nums[i] as i64;
            let mut t3 = t2 - 1;

            let mut sum2 = 0i64;
            let mut cnt2 = 0i64;

            if t3 >= 0 {
                // vsum2[nums[i] as usize] = (vsum2[nums[i] as usize] + vcnt[t3 as usize] * (nums[i] as i64) % MOD + vsum[t3 as usize]) % MOD;
                // vcnt2[nums[i] as usize] = (vcnt2[nums[i] as usize] + vcnt[t3 as usize]) % MOD;

                sum2 = (sum2 + vcnt[t3 as usize] * (nums[i] as i64) % MOD + vsum[t3 as usize]) % MOD;
                cnt2 = (cnt2 + vcnt[t3 as usize]) % MOD;
            }

            t3 = t2 + 1;
            // vsum2[nums[i] as usize] = (vsum2[nums[i] as usize] + vcnt[t3 as usize] * (nums[i] as i64) % MOD + vsum[t3 as usize]) % MOD;
            // vcnt2[nums[i] as usize] = (vcnt2[nums[i] as usize] + vcnt[t3 as usize]) % MOD;

            // vsum2[nums[i] as usize] = (vsum2[nums[i] as usize] + nums[i] as i64) % MOD;
            // vcnt2[nums[i] as usize] = (vcnt2[nums[i] as usize] + 1) % MOD;

            sum2 = (sum2 + vcnt[t3 as usize] * (nums[i] as i64) % MOD + vsum[t3 as usize]) % MOD;
            cnt2 = (cnt2 + vcnt[t3 as usize]) % MOD;

            vsum[nums[i] as usize] = (vsum[nums[i] as usize] + sum2 + nums[i] as i64) % MOD;
            vcnt[nums[i] as usize] = (vcnt[nums[i] as usize] + 1 + cnt2) % MOD;

            // for j in 0..3 {
            //     println!("{} {} {} {}", vsum[j], vsum2[j], vcnt[j], vcnt2[j]);
            // }

            // std::mem::swap(&mut vsum, &mut vsum2);
            // std::mem::swap(&mut vcnt, &mut vcnt2);
        }

        let mut ans = 0i64;
        for i in 0..vsum.len() {
            ans = (ans + vsum[i]) % MOD;
        }

        ans as i32
    }
}


struct Solution {}

fn main()
{

    // let vi = [1,2,1].to_vec();
    // let vi = [3,4,5].to_vec();

    let vi = [40,30,27,15,34,28,29,15,67,22,38,92,8,47,28,33,61,24,48,16,3,31,16,95,27,60,6,4].to_vec();

    println!("ans: {:?}", Solution::sum_of_good_subsequences(vi));
}


