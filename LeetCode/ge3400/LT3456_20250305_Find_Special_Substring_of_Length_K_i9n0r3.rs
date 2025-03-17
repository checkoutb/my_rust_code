











// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.35MB
// Beats10.14%


impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {

        let s = s.into_bytes();

        let mut cnt = 0;
        for i in 0..s.len() {
            if i > 0 && s[i] == s[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            if cnt == k {
                if i + 1 == s.len() || s[i] != s[i + 1] {
                    return true;
                }
            }
        }

        false
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


