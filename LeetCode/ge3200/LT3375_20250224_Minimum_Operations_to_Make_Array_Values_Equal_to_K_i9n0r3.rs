





// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.26MB
// Beats73.33%



// > h    all equal
// > h   h
// second max      max -> second max -> new second max

// to k,  k must <= every element

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        if k > nums[0] {
            return -1;
        }

        let mut cnt = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                cnt += 1;
            }
        }

        if nums[0] == k {
            cnt -= 1;
        }

        cnt
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


