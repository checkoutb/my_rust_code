








// g


// error, 要前前个的。 v[i][j] == v[i+1][j-1] + op


// palindromic subseq ...

// ？？？？？？？？？


impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let sz1 = s.len();
        if k as usize >= sz1 / 2 {
            return sz1 as i32;
        }

        let mut vvi = vec![vec![0; k as usize + 1]; sz1];

        let s = s.into_bytes();
        for i in 0..s.len() {
            s[i] -= b'a';
        }
        
        for i in 0..s.len() { // length
            let mut vv2 = vec![vec![0; k as usize + 1]; sz1];

            for j in 0..s.len() {    // substr [j, j+i]
                if j + i >= s.len() {
                    break;
                }
                let t2 = ((s[i + j] + 26 - s[j]) % 26).min((s[j] + 26 - s[i + j]) % 26);  // op to let [i]==[j]
                for m in 0..k as usize + 1 {   // already use m op in [i+1,j-1]
                    if i == 0 {
                        vv2[j][j + i] = 1;
                        continue;
                    }
                    vv2[j][m] = vv2[j][m].max(vv2[j + 1][m]).max(vv2[j][m]);
                    if (m as i32 + t2) <= k {
                        // vv2[j][j + i] = vv2[j][j + i - 1].max(vv2[j + 1][j + i]);
                        vv2[j][m + t2] = vv2[j + 1][m] + 2;
                    } else {
                        // vv2[j][j + i] = vv2[j][j + i - 1].max(vv2[j + 1][j + i]);
                    }
                }
            }

            vvi = vv2;
        }
        
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


