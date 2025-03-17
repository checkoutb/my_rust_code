








// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.24MB
// Beats80.00%




// ? first last ?   distinct.

// [0, 0] 2  => 0  ....


impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut vi = vec![0i64; 51];

        for i in 0..nums.len() {
            if i + k as usize - 1 >= nums.len() {
                break;
            }
            for j in 0..k as usize {
                vi[nums[i + j] as usize] = vi[nums[i + j] as usize] | (1 << (i as i32));
            }
        }

        for i in (0..vi.len()).rev() {
            if vi[i] != 0 {
                if vi[i] & (vi[i] - 1) == 0 {
                    return i as i32;
                }
            }
        }

        -1
    }
}

impl Solution {
    pub fn largest_integer___eee(nums: Vec<i32>, k: i32) -> i32 {
        let sz1 = nums.len();
        if nums[0] == nums[nums.len() - 1] {
            return -1;
        }
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for i in 1..(sz1 - 1) {
            if nums[i] == nums[0] {
                cnt1 += 1;
            }
            if nums[i] == nums[nums.len() - 1] {
                cnt2 += 1;
            }
        }
        if cnt1 == 0 && cnt2 == 0 {
            return nums[0].max(nums[nums.len() - 1]);
        } else if cnt1 == 0 {
            return nums[0];
        } else if cnt2 == 0 {
            return nums[nums.len() - 1];
        } else {
            return -1;
        }
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


