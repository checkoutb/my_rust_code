
// D D

// for i in 1..((n/2)+1 as usize) {
//     if n % i != 0 {
//         continue;
//     }
//     let pattern = s.get(..i).unwrap().repeat((n / i) as usize);
//     if pattern == s {
//         return true;
//     }
// }


// Let S1 = S + S (where S in input string)
// Remove 1 and last char of S1. Let this be S2
// If S exists in S2 then return true else false



// Runtime3 ms
// Beats
// 79.63%
// Memory2.1 MB
// Beats
// 77.78%


// 质数个屁。。。

impl Solution {
    // 只想到 质数
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let sz1 = s.len();
        if sz1 == 1 {
            return false;
        }
        let mut cnt = 1;
        for i in 1..sz1 {
            if s[i] != s[i - 1] {
                cnt += 1;
                break;
            }
        }
        if cnt == 1 {
            return true;
        }
        let mut vb = vec![false; sz1 / 2 + 1];
        for i in 2..(vb.len()) {

            // println!("----{}, {}", i, vb.len());

            if vb[i] == true {
                continue;
            }
            if sz1 % i != 0 {
                continue;
            }
            // let mut j = i + i;
            // while j < vb.len() {
            //     vb[j] = true;
            //     j += i;
            // }
            let mut can = true;
            for m in 0..i {
                let mut n = m + i;

                // println!("{}, {}", m, n);

                while n < sz1 {
                    if s[m] != s[n] {
                        can = false;
                        break;
                    }
                    n += i;
                }

                if !can {
                    break;
                }
            }
            if can {
                return true;
            }
        }
        return false;
    }
}




struct Solution {}

fn main()
{

    // let s = "abab".to_string();
    let s = "abacababacab".to_string();

    println!("ans: {:?}", Solution::repeated_substring_pattern(s));
}


