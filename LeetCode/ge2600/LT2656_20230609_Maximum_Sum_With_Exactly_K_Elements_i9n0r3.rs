

// D D

// nums.iter().max().unwrap() * k + (k - 1) * k / 2





// Runtime8 ms
// Beats
// 32.46%
// Memory2.3 MB
// Beats
// 10.53%
impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mx = nums[nums.len() - 1];
        (mx + mx + k - 1) * k / 2

        // let sz1 = nums.len() as i32;
        // (nums[sz1 - 1] + nums[s])
        // let nums: Vec<i32> = nums.iter().rev().collect();
        // (nums[0] + nums[0] + k - 1) * (k) / 2
    }
}



struct Solution {}

fn main()
{

    let vi: Vec<i32> = [2,5,4,2,1].to_vec();
    let k = 3;

    println!("ans: {:?}", Solution::maximize_sum(vi, k));
}


