

// D D

//  impl Solution {
//     pub fn make_smallest_palindrome(s: String) -> String {
//       let l = s.len();
//       let mut s: Vec<char> = s.chars().collect();
//       for i in 0 .. l / 2 {
//         if s[i] != s[l - i - 1] {
//           let v = s[i].min(s[l - i - 1]);
//           s[i] = v;
//           s[l - i - 1] = v;
//         }
//       }
//       return s.into_iter().collect();
//     }
//  }




// Runtime4 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.2 MB
// Beats
// 100%

// s is immut...
// HOW TO CHANGE CHAR? NO UNSAFE.
impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut st: usize = 0;
        let mut en = s.len() - 1;

        let mut s = s.clone();          // ...

        let arr = unsafe { s.as_bytes_mut() };

        while st < en {
            if arr[st] < arr[en] {
                arr[en] = arr[st];
            } else {
                arr[st] = arr[en];
            }
            st += 1;
            en -= 1;
        }
        s
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::make_smallest_palindrome("egcfe".to_string()));
}


