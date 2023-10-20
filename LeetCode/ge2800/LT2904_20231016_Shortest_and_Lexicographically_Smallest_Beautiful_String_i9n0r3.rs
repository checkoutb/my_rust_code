





// Runtime0 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.1 MB
// Beats
// 100%

// shortest, if multi, return lexi min
impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let mut mxsz = usize::MAX;
        let mut mnst = 0;    // start index
        let mut cnt = 0;
        let mut st = 0;
        let mut ori_s = s.clone();
        let s = s.as_bytes();
        for en in 0..s.len() {
            if s[en] == b'1' {
                cnt += 1;
            }
            if cnt == k {
                while s[st] == b'0' {
                    st += 1;
                }
                let tsz = en - st + 1;
                if tsz < mxsz {
                    mxsz = tsz;
                    mnst = st;
                } else if tsz == mxsz && Self::aSmall(s, st, mnst, en) {
                    mnst = st;
                }
                st += 1;
                cnt -= 1;
            }
        }
        if mxsz == usize::MAX {
            return "".to_string();
        }
        ori_s.split_off(mnst + mxsz);
        ori_s.split_off(mnst)
    }

    fn aSmall(arr: &[u8], mut sta: usize, mut stb: usize, ena: usize) -> bool {
        while sta <= ena {
            if arr[sta] != arr[stb] {
                return arr[sta] < arr[stb];
            }
            sta += 1;
            stb += 1;
        }
        return false;
    }
}



struct Solution {}

fn main()
{

    let s = "100011001".to_string();
    let k = 3;

    println!("ans: {:?}", Solution::shortest_beautiful_substring(s, k));
}


