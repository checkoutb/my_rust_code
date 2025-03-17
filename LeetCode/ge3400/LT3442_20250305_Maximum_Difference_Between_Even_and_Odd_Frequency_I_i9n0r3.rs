








// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.22MB
// Beats46.74%



// max odd - min even

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let s = s.into_bytes();
        let au8 = b'a';
        let mut vi = vec![0; 26];
        for i in 0..s.len() {
            vi[(s[i] - au8) as usize] += 1;
        }
        let mut mxodd = 0;
        let mut mneven = s.len() as i32;
        for i in 0..vi.len() {
            if vi[i] == 0 {
                continue;
            }
            if vi[i] % 2 == 0 {
                mneven = mneven.min(vi[i]);
            } else {
                mxodd = mxodd.max(vi[i]);
            }
        }
        mxodd - mneven
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


