



// D D

// public long GCD(long x, long y)
// {
//     while (x > 0 && y > 0)
//     {
//         if (x > y)
//         {
//             x %= y;
//         }
//         else
//         {
//             y %= x;
//         }
//     }
//     return x | y;
// }




// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.16MB
// Beats59.57%


// sz < 100, 硬算？  pfx sfx
// 还是说 移除最小的？

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let mut pfxgcd = vec![0i64; nums.len()];
        let mut pfxlcm = vec![0i64; nums.len()];
        pfxgcd[0] = nums[0] as i64;
        pfxlcm[0] = nums[0] as i64;
        let mut t2 = 0i64;
        for i in 1..nums.len() {
            t2 = Self::cal_gcd(pfxgcd[i - 1], nums[i] as i64) as i64;
            pfxgcd[i] = t2;
            t2 = Self::cal_gcd(pfxlcm[i - 1], nums[i] as i64);
            pfxlcm[i] = nums[i] as i64 / t2 * pfxlcm[i - 1];
        }

        let mut sfxgcd = nums[nums.len() - 1] as i64;
        let mut sfxlcm = sfxgcd;
        let mut ans = pfxgcd[pfxgcd.len() - 1] * pfxlcm[pfxlcm.len() - 1];

        for i in (0..nums.len()).rev() {  // remove i
            // let mut t2 = 1i64;
            if i + 1 == nums.len() && i > 0 {
                ans = ans.max(pfxgcd[i - 1] * pfxlcm[i - 1]);
            } else if i == 0 {
                ans = ans.max(sfxgcd * sfxlcm);
            } else {
                // ? 可以直接乘吗？
                let t2 = Self::cal_gcd(sfxgcd, pfxgcd[i - 1]) as i64;
                let t3 = Self::cal_gcd(sfxlcm, pfxlcm[i - 1]) as i64;
                let t3 = sfxlcm / t3 * pfxlcm[i - 1];
                ans = ans.max(t2 * t3);

                let t2 = Self::cal_gcd(nums[i] as i64, sfxgcd) as i64;
                let t3 = Self::cal_gcd(nums[i] as i64, sfxlcm) as i64;

                sfxgcd = t2;
                sfxlcm = sfxlcm / t3 * nums[i] as i64;

            }
        }
        ans
    }

    fn cal_gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }
        Self::cal_gcd(b, a % b)
    }
}


struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


