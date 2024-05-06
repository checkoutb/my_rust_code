






// int dist_a = min(s[i] - 'a', 'z' - s[i] + 1);
// s[i] = dist_a <= k ? 'a' : s[i] - k;
// k -= dist_a;

// Runtime
// 2ms
// Beats63.16%of users with Rust
// Memory
// 2.22MB
// Beats5.26%of users with Rust
impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut vc : Vec<char> = s.chars().collect();
        let au32 = 'a' as u32;
        let zu32 = 'z' as u32;
        let mut k = k;
        for i in 0..vc.len() {
            if (k == 0) {
                break;
            }
            let mut t2 = vc[i] as u32;
            let tp = t2 - au32;
            let ts = zu32 - t2 + 1;
            if (k as u32) < ts {
                if (k as u32) >= tp {
                    k -= tp as i32;
                    vc[i] = 'a';
                } else {
                    use std::char::from_u32;
                    vc[i] = from_u32(t2 - (k as u32)).unwrap();
                    k = 0;
                }
            } else {
                k -= ts.min(tp) as i32;
                vc[i] = 'a';
            }
        }

        vc.iter().collect::<String>()
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


