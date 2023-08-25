

// D D

// if s3.len() != s1.len() + s2.len() {
//     return false;
// }
// let (b1, b2, b3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
// let (n1, n2) = (b1.len(), b2.len());
// let mut table: Vec<Vec<bool>> = Vec::new();
// for _ in 0..=n1 {
//     let mut v: Vec<bool> = Vec::new();
//     for _ in 0..=n2 {
//         v.push(false);
//     }
//     table.push(v);
// }
// table[n1][n2] = true;
// for i in (0..=n1).rev() {
//     for j in (0..=n2).rev() {
//         if i < n1 && b1[i] == b3[i + j] && table[i + 1][j] {
//             table[i][j] = true;
//         } else if j < n2 && b2[j] == b3[i + j] && table[i][j + 1] {
//             table[i][j] = true;
//         }
//     }
// }



// Runtime1 ms
// Beats
// 90%
// Memory2.2 MB
// Beats
// 30%


// 天之上。。。
// 现在想来，一种dp，第二种就是 直接在s3中找s1,然后剩下的是不是s2.
// dp应该可以，但是有点模糊。
// 。。第二种不行。 aba + ab = ab aba  但是找s1的话，剩下的就是 ba了
// dp了

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let sz1 = s1.len();
        let sz2 = s2.len();
        let sz3 = s3.len();
        if sz1 + sz2 != sz3 {
            return false;
        }
        if sz1 == 0 {
            return s2 == s3;
        }
        if sz2 == 0 {
            return s1 == s3;
        }
        if sz3 == 0 {
            return true;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        let mut vvi = vec![vec![false; sz2 + 1]; sz1 + 1];
        // let mut vvi = vec![vec![false; sz1]; sz2];
        vvi[0][0] = true;

        for k in 1..=sz3 {      // count,  index + 1
            let mut mni = 0;     // count
            if k > sz2 {
                mni = k - sz2;
            }
            let mxi = sz1.min(k);
            for i in mni..=mxi {
                let j = k - i;      // count
                if i == 0 {
                    if s2[j - 1] == s3[k - 1] && vvi[i][j - 1] {
                        vvi[i][j] = true;
                    }
                } else if j == 0 {
                    if s1[i - 1] == s3[k - 1] && vvi[i - 1][j] {
                        vvi[i][j] = true;
                    }
                } else {

                    // println!("{}, {}, {}", i, j, k);

                    if s1[i - 1] == s3[k - 1] && vvi[i - 1][j] {
                        vvi[i][j] = true;
                    } else if s2[j - 1] == s3[k - 1] && vvi[i][j - 1] {
                        vvi[i][j] = true;
                    }
                }
            }
        }
        vvi[sz1][sz2]

        // for i in 0..=sz1 {
        //     for j in 0..=sz2 {
        //         if i == 0 {
        //             if j > 0 {
        //                 if vvi[0][j - 1] && s2[j - 1] == s3[j - 1] {
        //                     vvi[0][j] = true;
        //                 }
        //             }
        //         } else if j == 0 {
        //             if vvi[i - 1][0] && s1[i - 1] == s3[i - 1] {
        //                 vvi[i][0] = true;
        //             }
        //         } else {
        //             if s1[i - 1] == s3[i + j - 2] && vvi[i - 1][j] {
        //                 vvi[i][j] = true;
        //             } else if s2[j - 1] == s3[i + j - 2] && vvi[i][j - 1] {
        //                 vvi[i][j] = true;
        //             }
        //         }
        //     }
        // }

        // println!("{:?}", vvi);

        // return vvi[sz1][sz2];

        // for i in 0..sz3 {       // s3 index
        //     let mxi = (i + 1).min(sz1);
        //     for j in 0..=mxi {        // s1's count , index + 1
        //         if j == 0 {
        //             if i < sz2 {
        //                 if s2[i] == s3[i] && vvi[0][i] {
        //                     vvi[0][i] = true;
        //                 }
        //             }
        //         // } else if j == mxi {
        //         //     if i < sz1 {
        //         //         if s1[i] == s3[i] && vvi[i][0] {
        //         //             vvi[i][0] = true;
        //         //         }
        //         //     }
        //         } else {
        //             if s1[j - 1] == s3[i] && vvi[j - 1][i + 1 - j] {
        //                 vvi[][] = true;
        //             } else if (s2[] == s3[i] && vvi[][]) {
        //                 vvi[][] = true;
        //             }
        //         }

        //         // if j > 0 && s1[j - 1] == s3[i] {
        //         //     if vvi[j - 1][i - j + 1] {
        //         //         vvi[j][i - j] = true;
        //         //     }
        //         // }
        //         // if j < i && s2[i - j] = s3[i] {

        //         // }
        //         // if j == 0 && s2[i] = s3[i] {
        //         //     if vvi[0][i - 1] {
        //         //         vvi[0][i] = true;
        //         //     }
        //         // }
        //         // if j == i && s1[i] == s3[j] {
        //         //     if vvi[i - 1][0] {
        //         //         vvi[i][0] = true;
        //         //     }
        //         // }
        //     }
        // }
        // return vvi[sz1][sz2];
    }
}



//[[true, false, false, false, false, false], 
// [true, false, false, false, false, false], 
// [true, false, false, false, false, false], 
// [false, false, false, false, false, false], 
// [false, false, false, false, false, false], 
// [false, false, false, false, false, false]]


struct Solution {}

fn main()
{
    // let s1 = "aabcc".to_string();
    // let s2 = "dbbca".to_string();
    // let s3 = "aadbbbaccc".to_string();

    let s1 = "db".to_string();
    let s2 = "b".to_string();
    let s3 = "cbb".to_string();

    println!("ans: {:?}", Solution::is_interleave(s1, s2, s3));
}


