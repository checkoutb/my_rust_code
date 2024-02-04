
// D D

// let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
// let mut cs1 = s1.chars();
// let mut cs2 = s2.chars();
// let mut cs3 = s3.chars();
// if let (Some(c1), Some(c2), Some(c3)) = (cs1.next(), cs2.next(), cs3.next()) {
//     if !(c1 == c2 && c2 == c3) {
//         return -1;
//     }
// }
// let mut res = (n1 + n2 + n3) as i32 - 3;
// while let (Some(c1), Some(c2), Some(c3)) = (cs1.next(), cs2.next(), cs3.next()) {
//     if !(c1 == c2 && c2 == c3) {
//         break;
//     }
//     res -= 3;
// }





// Runtime3 ms
// Beats
// 38.46%
// Memory2.2 MB
// Beats
// 30.77%
impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        // let sz1 = s1.len();
        // let sz2 = s2.len();
        // let sz3 = s3.len();
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (sz1, sz2, sz3) = (s1.len(), s2.len(), s3.len());
        let szmn = sz1.min(sz2.min(sz3));
        let mut t2 = 0usize;

        while t2 < szmn {
            if s1[t2] != s2[t2] || s2[t2] != s3[t2] {
                break;
            }
            t2 += 1;
        }

        if t2 == 0 {
            return -1;
        }

        return (sz1 + sz2 + sz3 - 3 * t2) as i32;
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


