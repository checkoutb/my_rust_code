

// D D

// match t.chars().nth(0)
// {
//     Some(result) => return result,
//     None => return ' '
// }



// let char1 = s.as_bytes();
// let char2 = t.as_bytes();
// let sum: u32 = char1.iter().map(|&byte| byte as u32).sum();
// let sum2: u32 = char2.iter().map(|&byte| byte as u32).sum();
// let result1 = sum2 - sum;
// let binding = [result1 as u8];
// let result2 = std::str::from_utf8(&binding);
// match result2 {
//     Ok(s) => {
//         match s.chars().nth(0) {
//             Some(result) => return result,
//             None => return ' '
//         }
//     }
//     Err(e) => {
//         println!("{}", e)
//     }
// }
// return ' ';



// for (int i = 0; i < s.length(); ++i) {
//     c ^= s.charAt(i);
// }
// for (int i = 0; i < t.length(); ++i) {
//     c ^= t.charAt(i);
// }





// Runtime0 ms
// Beats
// 100%
// Memory2.2 MB
// Beats
// 12.21%

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut vi = vec![0; 26];
        let au8: u8 = b'a';
        for i in 0..s.len() {
            vi[(s[i] - au8) as usize] -= 1;
            vi[(t[i] - au8) as usize] += 1;
        }
        vi[(t[t.len() - 1] - au8) as usize] += 1;
        for i in 0..26 {
            if vi[i] != 0 {
                // println!("qqqq {},{} pppp", (i + au8 as usize) as u32, char::from_digit((i + au8 as usize) as u32, 10).unwrap_or_default());
                // println!("qqqq {},{} pppp", (i + au8 as usize) as u32, char::from_u32((i + au8 as usize) as u32).unwrap_or_default());
                return char::from_u32((i + au8 as usize) as u32).unwrap();
            }
        }
        return '-';
    }
}




struct Solution {}

fn main()
{

    let s = "asdqwezxc".to_string();
    let t = "asdqwezxcz".to_string();

    println!("ans: {:?}", Solution::find_the_difference(s, t));
}


