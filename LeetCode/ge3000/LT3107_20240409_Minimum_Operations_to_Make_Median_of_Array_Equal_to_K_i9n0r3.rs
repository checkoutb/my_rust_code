





// Runtime
// 36ms
// Beats19.05%of users with Rust
// Memory
// 4.95MB
// Beats76.19%of users with Rust
impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort();
        let mut md = nums[nums.len() / 2];
        let mut ans = 0i64;
        if md > k {
            let mut idx = nums.len() / 2;
            while idx >= 0 {
                if nums[idx] > k {
                    ans += (nums[idx] - k) as i64;
                } else {
                    break;
                }
                // idx -= 1;
                if idx == 0 {
                    break;
                } else {
                    idx -= 1;
                }
            }
        } else {
            let mut idx = nums.len() / 2;
            while idx < nums.len() {
                if nums[idx] < k {
                    ans += (k - nums[idx]) as i64;
                } else {
                    break;
                }
                idx += 1;
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


