







// Runtime
// 18ms
// Beats100.00%of users with Rust
// Memory
// 3.60MB
// Beats100.00%of users with Rust
impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut map2 = HashMap::new();
        let mut idx = 0usize;
        while idx < word.len() {
            map2.entry(&word[idx .. (idx + k as usize)]).and_modify(|cnt| *cnt += 1).or_insert(1);
            idx += k as usize;
        }
        (word.len() as i32 / k) - map2.into_values().collect::<Vec<_>>().iter().max().unwrap()
    }
}


struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


