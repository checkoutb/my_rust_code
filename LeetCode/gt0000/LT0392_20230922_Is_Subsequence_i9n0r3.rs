
// D D

// let mut i = 0;
// let mut j = 0;

// while i < s.len() && j < t.len() {
//     if s.as_bytes()[i] == t.as_bytes()[j] {
//         i+=1
//     }
//     j+=1
// }
// i == s.len()




// Runtime0 ms
// Beats
// 100%
// Memory2.3 MB
// Beats
// 12.88%

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut sidx = 0 as usize;
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..t.len() {
            if s[sidx] == t[i] {
                sidx += 1;
                if sidx == s.len() {
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


