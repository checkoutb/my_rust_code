






// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.12MB
// Beats68.57%of users with Rust


// 1 : up,  0:== , -1: down

impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut vi = vec![0; nums.len() - 1];
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                vi[i - 1] = 1;
            } else if nums[i] == nums[i - 1] {
                vi[i - 1] = 0;
            } else {
                vi[i - 1] = -1;
            }
        }
        let mut ans = (vi.len() - pattern.len()) as i32 + 1;
        for i in 0..=(vi.len() - pattern.len()) {
            for j in 0..pattern.len() {
                if pattern[j] != vi[i + j] {
                    ans -= 1;
                    break;
                }
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


