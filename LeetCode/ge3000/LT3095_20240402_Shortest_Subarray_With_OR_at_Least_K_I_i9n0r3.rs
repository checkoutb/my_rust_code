









// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.11MB
// Beats29.41%of users with Rust
// 50 * 50
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            let mut t2 = nums[i];
            if t2 >= k {
                ans = 1;
                break;
            }
            for j in (i + 1)..nums.len() {
                t2 |= nums[j];
                if t2 >= k {
                    ans = ans.min((j - i + 1) as i32);
                    break;
                }
            }
        }
        if ans == i32::MAX {
            ans = -1;
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [1,2,3].to_vec();
    let k = 2;

    println!("ans: {:?}", Solution::minimum_subarray_length(vi, k));
}


