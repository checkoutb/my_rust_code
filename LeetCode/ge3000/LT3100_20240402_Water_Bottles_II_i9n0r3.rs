







// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.03MB
// Beats63.64%of users with Rust
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut t2 = num_bottles;
        let mut num_exchange = num_exchange;
        while t2 >= num_exchange {
            ans += 1;
            t2 -= num_exchange;
            t2 += 1;
            num_exchange += 1;
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


