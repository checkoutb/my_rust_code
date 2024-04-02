






// Runtime
// 9ms
// Beats88.89%of users with Rust
// Memory
// 2.80MB
// Beats44.44%of users with Rust
impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut i = 0usize;
        let mut ans = 0i64;
        while i < nums.len() {
            let mut en = i;
            while en + 1 < nums.len() && nums[en + 1] != nums[en] {
                en += 1;
            }
            ans += (en - i + 2) as i64 * (en - i + 1) as i64 / 2i64;
            i = en + 1;
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


