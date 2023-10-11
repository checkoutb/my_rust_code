
// D D

// We count "zeros" so far. If we have a string 00001, we need 4 seconds (as we have 4 zeros) to make it 10000.

// We also track seconds we need to rearrange first i letters.

// If i + 1 letter is 1, we need at least seconds + 1 seconds, but not less than zero seconds.

// int zeros = 0, seconds = 0;
// for (int i = 0; i < s.size(); ++i) {
//     zeros += s[i] == '0';
//     if (s[i] == '1' && zeros)
//         seconds = max(seconds + 1, zeros);
// }
// return seconds;




// Sorry, there are not enough accepted submissions to show data
// Runtime16 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.3 MB
// Beats
// 100%


// 01 10
// 1st 0, after all 1
// ...
impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {

        let mut s = s;
        let s = unsafe { s.as_bytes_mut() };

        let mut ans = 0;
        loop {
            let mut i = 0;
            let mut chg = false;
            while i < s.len() - 1 {
                if s[i] == b'0' && s[i + 1] == b'1' {
                    s.swap(i, i + 1);
                    // s[i] = b'1';
                    // s[i + 1] = b'0';
                    i += 2;
                    chg = true;
                } else {
                    i += 1;
                }
            }
            if !chg {
                break;
            }
            ans += 1;
        }
        ans

        // let cnt = 0;    // count 1
        // let idx = -1;   // index of first 0

        // let s = s.as_bytes();
        // let mut t2 = 0;
        // let mut lst0 = -1;
        // let mut ans = 0;
        // for i in 0..s.len() {
        //     if s[i] == b'1' {
        //         t2 += 1;
        //         if lst0 == -1 {
        //             continue;
        //         }
        //         println!("{}, {}, {}, {}", i, t2, lst0, 1);
        //         ans = ans.max(i - t2 + i - lst0 as usize);
        //     } else {
        //         lst0 = i as i32;
        //     }
        //     // if s[i] == '0' && idx == -1 {
        //     //     idx = i as i32;
        //     // }
        //     // if s[i] == '1' {
        //     //     cnt += 1;
        //     // }
        // }
        // ans as i32
    }
}



struct Solution {}

fn main()
{

    let s = "0110101".to_string();

    println!("ans: {:?}", Solution::seconds_to_remove_occurrences(s));
}


