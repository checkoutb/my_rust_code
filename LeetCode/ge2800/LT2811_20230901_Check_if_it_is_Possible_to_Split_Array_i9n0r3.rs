




// Runtime2 ms
// Beats
// 68.75%
// Memory2.1 MB
// Beats
// 47.92%

// [i]+[i+1] >= m ?
impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        if nums.len() <= 2 {            // ....
            return true;
        }
        for i in 1..nums.len() {
            if nums[i] + nums[i - 1] >= m {
                return true;
            }
        }
        return false;
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


