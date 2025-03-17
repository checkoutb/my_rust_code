








// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.24MB
// Beats100.00%




// ? 删除的元素index必须连续？ no

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut vb = vec![false; 101];
        let mut sum2 = 0;
        let mut mxless1 = -101;
        for i in 0..nums.len() {
            if nums[i] > 0 {
                if !vb[nums[i] as usize] {
                    sum2 += nums[i];
                    vb[nums[i] as usize] = true;
                }
            } else {
                mxless1 = mxless1.max(nums[i]);
            }
        }

        if sum2 == 0 {
            mxless1
        } else {
            sum2
        }
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


