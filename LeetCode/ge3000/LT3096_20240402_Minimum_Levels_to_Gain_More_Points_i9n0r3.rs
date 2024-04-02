






// Runtime
// 33ms
// Beats30.95%of users with Rust
// Memory
// 2.97MB
// Beats61.90%of users with Rust
impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut vi = vec![0i32; possible.len()];
        let mut idx = possible.len();
        while idx > 0 {
            if idx != possible.len() {
                vi[idx - 1] = vi[idx] + (if possible[idx - 1] == 1 {1} else {-1});
            } else {
                vi[idx - 1] = if possible[idx - 1] == 1 {1} else {-1};
            }
            idx -= 1;
        }
        let mut got = 0i32;
        for i in 0..(possible.len() - 1) {
            got = got + (if possible[i] == 1 { 1 } else { -1 });
            if got > vi[i + 1] {
                return i as i32 + 1;
            }
        }
        -1
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


