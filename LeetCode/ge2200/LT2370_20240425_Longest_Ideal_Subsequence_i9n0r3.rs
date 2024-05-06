







// Runtime
// 6ms
// Beats100.00%of users with Rust
// Memory
// 2.40MB
// Beats60.00%of users with Rust
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut vi: Vec<i32> = vec![-1; 26];
        let s = s.as_bytes();
        let au8 = b'a' as usize;
        for i in 0..s.len() {
            let u = s[i];
            let mut mx = 0;
            for j in 0.max(u as i32 - k - au8 as i32) .. 26.min(u as i32 + k + 1 - au8 as i32) {
                // if vi[j] != -1 {
                    mx = mx.max(vi[j as usize]);
                // }
            }
            vi[u as usize - au8] = mx + 1;
        }
        let mut ans = 0;
        for i in 0..26 {
            ans = ans.max(vi[i]);
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


