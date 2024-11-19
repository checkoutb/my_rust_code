






// Runtime
// 21ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.45MB
// Beats100.00%


impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0i32;
        for i in 0..(nums.len() - 2) {
            if (nums[i] == 0) {
                nums[i + 1] = 1 - nums[i + 1];
                nums[i + 2] = 1 - nums[i + 2];
                ans += 1;
            }
        }
        if nums[nums.len() - 1] == 1 && nums[nums.len() - 2] == 1 {
            return ans;
        } else {
            return -1i32;
        }
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


