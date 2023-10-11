

// impl Solution {
//     pub fn shifting_letters(mut s: String, shifts: Vec<Vec<i32>>) -> String {
//         let mut suffix_add = vec![0; s.len() + 1];
//         for mut shift in shifts {
//             shift[2] += shift[2] - 1;   // turning 0 into -1 (and leaving 1 as 1)
//             suffix_add[shift[0] as usize] += shift[2];  // applying change for [beg;inf)
//             suffix_add[shift[1] as usize + 1] -= shift[2];  // applying  reversed change for (end; inf)
//         }
//         unsafe {    // Code is unsafe because of as_bytes_mut() - but since we are ensured that input is only lowecase english alphabet everything is safe
//             let s = s.as_bytes_mut();
//             let mut it = suffix_add.into_iter();    // We create iterator over suffix_add but it would be perfectly fine to use additional variable to keep track of current position
//             let mut n = it.next().unwrap();                     
//             for byte in s {                                 // ^... or we can simply enumerate
//                 *byte = ((((*byte as i32 - 97) + n)%26 + 26)%26) as u8 + 97;    // Whole magic :-) (Notes)
//                 n += it.next().unwrap();
//             }
//         }
//         s
//     }
// }


// Runtime23 ms
// Beats
// 100%
// Memory5.8 MB
// Beats
// 100%

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        use std::borrow::Cow;

        let mut vi = vec![0; s.len() + 1];
        for shf in shifts.iter() {
            if shf[2] == 1 {
                vi[shf[0] as usize] += 1;
                vi[shf[1] as usize + 1] -= 1;
            } else {
                vi[shf[0] as usize] -= 1;
                vi[shf[1] as usize + 1] += 1;
            }
        }
        let mut s = s;
        let s = unsafe { s.as_bytes_mut() };
        let mut t2 = 0;
        for i in 0..s.len() {
            t2 += vi[i];
            s[i] = (((t2 + (s[i] as i32) - (b'a' as i32)) % 26 + 26) % 26 + (b'a' as i32)) as u8;
        }
        // String::from_utf8(s).unwrap().to_string()
        // Cow::Owned(String::from_utf8_lossy(&s))
        String::from_utf8_lossy(&s).to_string()
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


