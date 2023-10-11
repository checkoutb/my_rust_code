




// Runtime37 ms
// Beats
// 50%
// Memory4.1 MB
// Beats
// 50%

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let t2 = (nums.len() - 1) as i32;
        let mut ans = t2 as usize;
        let mut en = 0;
        let mut t3 = 0;
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                t3 -= 1;
            }
            while en < nums.len() && nums[en] <= nums[i] + t2 {
                if en > i && nums[en] == nums[en - 1] {
                    t3 += 1;
                }
                en += 1;
            }
            ans = ans.min(nums.len() - (en - i) + t3);
        }
        ans as i32
    }
}




struct Solution {}

fn main()
{
    // let vi = [4,3,5,2].to_vec();
    // let vi = [1,3,2,5,6].to_vec();
    let vi = [8,5,9,9,8,4].to_vec();

    println!("ans: {:?}", Solution::min_operations(vi));
}


