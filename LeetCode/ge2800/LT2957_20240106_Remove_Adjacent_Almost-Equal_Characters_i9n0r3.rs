

// D D

// int k = 0, n = s.length();
// for (int i = 1; i + k < n; ++i)
//     k += Math.abs(s.charAt(i + k) - s.charAt(i + k - 1)) < 2 ?  1 : 0;
// return k;


// abs <= 1



// Runtime2 ms
// Beats
// 55.56%
// Memory2.1 MB
// Beats
// 19.44%

impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {

        fn almost_eq(a: u8, b: u8) -> bool {
            a == b || (a + 1) == b || a == (b + 1)
        }

        let mut ans = 0;

        let s = word.as_bytes();

        let mut idx = 1;
        while idx < s.len() {
            if almost_eq(s[idx], s[idx - 1]) {
                ans += 1;
                idx += 2;
            } else {
                idx += 1;
            }
        }

        ans
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


