





// Runtime0 ms
// Beats
// 100%
// Memory2.2 MB
// Beats
// 25%
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {

        // topic: monostack ...

        use std::collections::VecDeque;

        let s = s.as_bytes();
        let au8 = b'a';
        let mut vi = [0; 26];  // last appear
        for i in 0..s.len() {
            vi[(s[i] - au8) as usize] = i as usize;
        }
        let mut stk = VecDeque::new();
        let mut v2 = [0; 26];    // count in stack

        for i in 0..s.len() {
            // while !stk.is_empty() && stk.
            // while stk.back().unwrap_or('z') > s[i]
            if v2[(s[i] - au8) as usize] != 0 {
                continue;
            }
            while !stk.is_empty() && *(stk.back().unwrap()) as u8 > s[i] && vi[(stk.back().unwrap() - au8) as usize] > i {
                v2[(stk.back().unwrap() - au8) as usize] -= 1;
                stk.pop_back();
            }
            v2[(s[i] - au8) as usize] += 1;
            stk.push_back(s[i]);
        }

        let mut ans = String::new();
        while !stk.is_empty() {
            ans.push(char::from_u32(*stk.front().unwrap() as u32).unwrap());
            stk.pop_front();
        }
        ans



        // let s = s.as_bytes();
        // let au8 = b'a';
        // let mut vi = [-1; 26];      // first appear
        // let mut v2 = [-1; 26];      // last appear
        // for i in 0..s.len() {
        //     if vi[(s[i] - au8) as usize] == -1 {
        //         vi[(s[i] - au8) as usize] = i as i32;
        //     }
        //     v2[(s[i] - au8) as usize] = i as i32;
        // }

        // let mut ans = String::new();
        // let mut lst = 0 as usize;
        // for _i in 0..26 {        // ans's max len
        //     for j in 0..26 {    // a-z
        //         if vi[j] == -1 {
        //             continue;
        //         }
        //         let mut can = true;
        //         for k in (j + 1)..26 {
        //             if v2[k] != -1 && v2[k] < vi[j] {
        //                 can = false;
        //                 break;
        //             }
        //         }
        //         if can {
                    
        //             println!("vi {:?},   {}", vi, j);
        //             println!("v2 {:?}", v2);

        //             vi[j] = -1;
        //             v2[j] = -1;
        //             ans.push(char::from_u32((j + au8 as usize) as u32).unwrap());
        //             break;
        //         }
        //     }
        // }
        // ans


        // // for 
        // // s.into_iter().map(|c| vi[*c as usize - au8 as usize] = 1);
        // s.into_iter().for_each(|c| vi[*c as usize - au8 as usize] = 1);

        // let mut ans = "".to_string();
        // for i in 0..26 {
        //     if vi[i] != 0 {
        //         ans.push(char::from_u32((i + au8 as usize) as u32).unwrap());
        //     }
        // }
        // ans
    }
}





struct Solution {}

fn main()
{

    let s = "cbacdcbcabcabacbacb".to_string();

    println!("ans: {:?}", Solution::remove_duplicate_letters(s));
}


