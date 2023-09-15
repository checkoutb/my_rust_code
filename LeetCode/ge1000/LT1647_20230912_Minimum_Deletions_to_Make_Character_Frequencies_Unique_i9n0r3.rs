






// Runtime5 ms
// Beats
// 16.67%
// Memory2.3 MB
// Beats
// 33.33%
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::HashSet;

        let s = s.as_bytes();
        let mut vi = vec![0; 26];
        let au8: u8 = b"a"[0];
        for i in 0..s.len() {
            vi[(s[i] - au8) as usize] += 1;
        }
        
        let mut set2 = HashSet::new();
        let mut ans = 0;
        for i in 0..26 {
            if vi[i] != 0 {
                while vi[i] > 0 && set2.contains(&vi[i]) {
                    vi[i] -= 1;
                    ans += 1;
                }
                if vi[i] != 0 {
                    set2.insert(vi[i]);
                }
            }
        }

        ans
    }
}



struct Solution {}

fn main()
{

    let s = "aaabbbcc".to_string();

    println!("ans: {:?}", Solution::min_deletions(s));
}


