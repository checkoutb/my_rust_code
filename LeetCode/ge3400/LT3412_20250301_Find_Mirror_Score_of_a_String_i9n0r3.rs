

// D D

// if let Some(x) = unmarked[mirror].pop() {
//     score += i - x;
// } else {
//     unmarked[ch].push(i);
// }




// Runtime
// 7ms
// Beats50.00%
// Memory
// 2.83MB
// Beats34.62%

// ->     
// find closest j < i  sj is si's mirror.  mark j i, add i-j

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let s = s.into_bytes();
        let au8 = b'a';
        let mut ans = 0;
        let mut vvi = vec![vec![]; 26];  // converted
        for i in 0..s.len() {
            let t2 = Self::converta1(s[i] - au8);
            if !vvi[t2].is_empty() {
                ans += (i - vvi[t2].pop().unwrap()) as i64;
            } else {
                vvi[(s[i] - au8) as usize].push(i);
            }
        }
        ans
    }

    fn converta1(ch: u8) -> usize {
        let mut ans = ch as usize;
        ans = 25 - ans;
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


