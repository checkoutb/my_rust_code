




// II
// Runtime
// 7ms
// Beats52.54%of users with Rust
// Memory
// 2.44MB
// Beats37.29%of users with Rust


// I
// Runtime
// 6ms
// Beats23.08%of users with Rust
// Memory
// 2.09MB
// Beats73.08%of users with Rust

// ...... distinct letter .....

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let vc = word.as_bytes().to_vec();
        let mut vcnt = vec![0; 26];
        let ba = b'a';
        for c in vc {
            vcnt[(c - ba) as usize] += 1;
        }
        vcnt.sort_unstable();
        vcnt.reverse();
        let mut ans = 0;
        // for (i, n) in vcnt.into_iter().enumerate().rev() {
        for (i, n) in vcnt.into_iter().enumerate() {
            if n == 0 {
                break;
            }
            // println!("{}, {}, {}", n, i, ans);
            ans += n * (i as i32 / 8 + 1);
        }
        ans
    }
}



struct Solution {}

fn main()
{
    let s = "asdfgh".to_string();

    println!("ans: {:?}", Solution::minimum_pushes(s));
}


