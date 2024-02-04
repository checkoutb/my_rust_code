






// Runtime0 ms
// Beats
// 100%
// Memory2.4 MB
// Beats
// 55%

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let s = s.as_bytes();
        let mut ans = 0i64;
        let mut cnt = 0i64;
        for &ch in s {
            if ch == b'0' {         // or for ch in s { if *ch == b'0' {}}
                ans += cnt;
            } else {
                cnt += 1;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


