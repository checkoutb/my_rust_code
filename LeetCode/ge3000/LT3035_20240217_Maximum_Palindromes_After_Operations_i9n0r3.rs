





// Runtime
// 2ms
// Beats75.00%of users with Rust
// Memory
// 2.33MB
// Beats100.00%of users with Rust


// i == j...
// odd ?   26 odd

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut oddch = 0;
        let mut oddsz = 0;
        let mut vch = [0; 26];
        let sz1 = words.len();
        let mut vsz = Vec::new();
        vsz.resize(sz1, 0);
        let ba = b'a';
        for (i, str) in words.into_iter().enumerate() {
            vsz[i] = str.len();
            if str.len() % 2 == 1 {
                oddsz += 1;
            }
            let s = str.into_bytes();
            for ch in s {
                vch[(ch - ba) as usize] += 1;
            }
        }
        for i in 0..26 {
            if vch[i] % 2 == 1 {
                oddch += 1;
            }
        }

        // println!("{}, {}", oddch, oddsz);
        
        if oddch == oddsz {
            return vsz.len() as i32;
        } else if oddch < oddsz {
            let t2 = (oddsz - oddch) % 2;
            return vsz.len() as i32 - t2;
        } else {
            let mut t2 = oddch - oddsz;     // ......
            vsz.sort();
            vsz.reverse();
            // println!("{}, {:?}", t2, vsz);
            for i in 0..vsz.len() {
                t2 -= vsz[i] as i32;
                if vsz[i] % 2 == 1 {            // ..
                    t2 += 1;
                }
                if t2 <= 0 {
                    return (vsz.len() - i - 1) as i32;
                }
            }
        }
        -1      // unreachable
    }
}


struct Solution {}

fn main()
{
    // let vs = ["abbb".to_string(), "ba".to_string(), "aa".to_string()].to_vec();
    let vs = ["nulr".to_string(),"owdyq".to_string(),"ycjof".to_string(),"td".to_string(),
    "fzuz".to_string(),"avzi".to_string(),"pkmb".to_string(),"odpx".to_string(),"efcv".to_string(),
    "vx".to_string(),"qo".to_string(),"c".to_string()].to_vec();

    println!("ans: {:?}", Solution::max_palindromes_after_operations(vs));
}


