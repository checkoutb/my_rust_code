










// Runtime
// 46ms
// Beats61.36%
// Memory
// 2.53MB
// Beats52.27%



// at most k  ... middle

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let mut nn = 0;
        let mut ss = 0;
        let mut ee = 0;
        let mut ww = 0;

        let mut ans = 0;
        for i in 0..s.len() {
            match s[i] {
                b'N' => nn += 1,
                b'S' => ss += 1,
                b'W' => ww += 1,
                b'E' => ee += 1,
                _ => (),
            }

            let mut t2;
            if nn >= ss {
                if ee >= ww {
                    let t3 = (ss + ww).min(k);
                    t2 = nn + ee - (ss + ww - t3) + t3;
                } else {
                    let t3 = (ss + ee).min(k);
                    t2 = nn + ww - ss - ee + t3 + t3;
                }
            } else {
                if ee >= ww {
                    let t3 = (nn + ww).min(k);
                    t2 = ss + ee - nn - ww + t3 + t3;
                } else {
                    let t3 = (nn + ee).min(k);
                    t2 = ss + ww - nn - ee + t3 + t3;
                }
            }
            ans = ans.max(t2);

            // ......
            // let sum2 = i as i32 + 1;
            // let min2 = (nn + ss).min(nn + ee).min(nn + ww).min(ss + ee).min(ss + ww).min(ee + ww);
            // let t2 = min2.min(k);

            // ans = ans.max(sum2 - min2 - (min2 - t2) + t2);
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


