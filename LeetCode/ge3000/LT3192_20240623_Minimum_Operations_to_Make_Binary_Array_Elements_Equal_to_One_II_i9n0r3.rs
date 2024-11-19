






// Runtime
// 18ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.68MB
// Beats100.00%


impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut chg = false;
        let mut t2 = 0i32;
        let mut ans = 0i32;
        for i in 0..nums.len() {
            t2 = if chg { 1 - nums[i] } else { nums[i] };
            if t2 == 0 {
                ans += 1;
                chg = !chg;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


