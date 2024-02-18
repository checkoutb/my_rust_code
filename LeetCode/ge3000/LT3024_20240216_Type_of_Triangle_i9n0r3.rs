





// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.22MB
// Beats27.27%of users with Rust

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort();
        let mut ans;
        if nums[0] + nums[1] <= nums[2] {
            ans = String::from("none");
        } else if nums[0] == nums[2] {
            ans = String::from("equilateral");
        } else if nums[0] == nums[1] || nums[1] == nums[2] {
            ans = String::from("isosceles");
        } else {
            ans = String::from("scalene");
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


