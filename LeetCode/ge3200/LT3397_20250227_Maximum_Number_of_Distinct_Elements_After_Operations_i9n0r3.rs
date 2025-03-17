






// Runtime
// 26ms
// Beats66.67%
// Memory
// 3.34MB
// Beats100.00%



impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut mn = i32::MIN;
        let mut ans = 0;
        for i in 0..nums.len() {
            let t2 = (nums[i] - k).max(mn + 1);
            if t2 <= nums[i] + k {
                nums[i] = t2;
                mn = t2;
            }
        }

        nums.sort();
        ans = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                ans += 1;
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


