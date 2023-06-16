





// Runtime1 ms
// Beats
// 73.58%
// Memory2.1 MB
// Beats
// 35.85%

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        if nums.is_empty() {
            return ans;
        }
        let mut st: i32 = nums[0];
        for i in 1..=nums.len() {
            if i == nums.len() || (nums[i] != nums[i - 1] + 1) {
                if nums[i - 1] == st {
                    ans.push(st.to_string());
                } else {
                    ans.push(st.to_string() + "->" + &(nums[i - 1]).to_string());
                }
                if i != nums.len() {
                    st = nums[i];
                }
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{
    let vi: Vec<i32> = [0,1,2,4,5,7].to_vec();

    println!("ans: {:?}", Solution::summary_ranges(vi));
}


