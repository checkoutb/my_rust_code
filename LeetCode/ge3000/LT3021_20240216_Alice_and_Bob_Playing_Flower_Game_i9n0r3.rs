

// D D

// return m * n / 2;



// Runtime
// 1ms
// Beats62.79%of users with Rust
// Memory
// 2.12MB
// Beats23.26%of users with Rust

// odd ?

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let n : i64 = n as i64;
        let m : i64 = m as i64;
        let on = n / 2 + n % 2;      // odd n
        let en = n / 2;      // even n
        let om = m / 2 + m % 2;
        let em = m / 2;
        on * em + en * om
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


