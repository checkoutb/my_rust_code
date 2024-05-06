







// no enough data
// Runtime
// 2ms
// Beats100.00%of users with Rust
// Memory
// 2.08MB
// Beats100.00%of users with Rust
impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        let mut tag = 0i32;
        for ch in word.chars() {
            if !ch.is_alphanumeric() {
                return false;
            } else if ch.is_alphabetic() {
                let ch2 = ch.to_ascii_lowercase();
                // if (ch2 == 'a' || ch2 == 'e' || ch2 == 'i' || ch2 == 'o' || ch2 == 'u') {   // match
                //     tag |= (1 << 1);
                // } else {
                //     tag |= 1;
                // }
                tag |= 1 << match (ch2) {
                    'a' | 'e' | 'i' | 'o' | 'u' => 1,
                    _ => 0,
                }
            }
        }
        tag == 3
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


