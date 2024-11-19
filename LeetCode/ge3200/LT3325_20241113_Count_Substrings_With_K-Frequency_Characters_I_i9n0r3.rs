





// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.14MB
// Beats80.26%


impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut vi = vec![0; 26];
        const au8: u8 = b'a';
        // let mut cnt_ge_k = 0i32;
        let mut ans = 0i32;
        let mut st = 0usize;
        let s = s.as_bytes();
        let mut ok = false;
        for i in 0..s.len() {
            let idx = (s[i] - au8) as usize;
            vi[idx] += 1;
            if vi[idx] == k {
                ok = true;
                while vi[idx] == k {
                    let idx2 = (s[st] - au8) as usize;
                    vi[idx2] -= 1;
                    st += 1;
                }
                // println!("{}, {}", st, ans);
                // ans += (st) as i32;
            }
            if ok {
                ans += st as i32;
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{

    let s = "abacb".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::number_of_substrings(s, k));
}


