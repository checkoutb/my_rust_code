

// D D

// pub fn beautiful_substrings(s: String, k: i32) -> i32 {
//     fn is_vowel(b: u8) -> bool {
//         b == 'a' as u8 || b == 'e' as u8 || b == 'i' as u8 || b == 'o' as u8 || b == 'u' as u8
//     }


    // for (ix, ch) in s.into_iter().enumerate() {
    //     let is_vow = if matches!(ch, b'a' | b'e' | b'i' | b'o' | b'u') {
    //         1
    //     } else {
    //         0
    //     };


// Runtime6 ms
// Beats
// 89.13%
// Memory2.1 MB
// Beats
// 56.52%

// substr    2*2 %4
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let sz1 = s.len();
        let s = s.as_bytes();
        let mut vv = vec![0; sz1];      // <=idx, count of Vowel
        let mut vc = vec![0; sz1];      //                 Consonant
        if Self::is_vowel(s[0]) {
            vv[0] = 1;
        } else {
            vc[0] = 1;
        }
        for i in 1..sz1 {
            if Self::is_vowel(s[i]) {
                vv[i] = vv[i - 1] + 1;
                vc[i] = vc[i - 1];
            } else {
                vv[i] = vv[i - 1];
                vc[i] = vc[i - 1] + 1;
            }
        }
        let mut ans = 0;
        let mut en = 1usize;        // substr [st, en]
        while en < sz1 {
            if vv[en] == vc[en] && (vv[en] * vv[en]) % k == 0 {
                ans += 1;
            }
            let mut prev_st = en % 2;
            let mut t2;
            let mut t3;
            while prev_st < en {
                t2 = vv[en] - vv[prev_st];
                t3 = vc[en] - vc[prev_st];
                if t2 == t3 && (t2 * t3) % k == 0 {
                    ans += 1;
                }
                prev_st += 2;
            }
            en += 1;
        }

        ans
        // for i in 1..sz1 {
        //     for j in 0..(i - 1) {         // 
        //     }
        // }
    }

    fn is_vowel(ch: u8) -> bool {
        ch == b'a' || ch == b'e' || ch == b'i' || ch == b'o' || ch == b'u'
    }
}


struct Solution {}

fn main()
{

    let s = "baeyh".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::beautiful_substrings(s, k));
}


