






// 这是 2024年 在 LeetCode 上解题的 第100天。 有徽章

// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.24MB
// Beats34.71%

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if (nums[i] % 2 == nums[i - 1] % 2) {
                return false;
            }
        }
        true
    }
}

struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


