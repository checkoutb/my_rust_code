




// Runtime
// 2ms
// Beats33.33%of users with Rust
// Memory
// 2.06MB
// Beats92.86%of users with Rust

// cut, starts_with, cut, starts_with ...

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let mut ans = 0;
        loop {
            ans += 1;
            if ans * k >= word.len() as i32 {
                break;
            }
            let s = &word[(ans * k) as usize ..];
            if word.starts_with(s) {
                break;
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{
    let s = "abacaba".to_string();
    let k = 3;

    println!("ans: {:?}", Solution::minimum_time_to_initial_state(s, k));
}


