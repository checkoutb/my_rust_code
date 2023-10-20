

// D D

// int mini = 0, maxi = 0, n = A.length;
// for (int i = d; i < n; i++) {
//     if (A[i - d] < A[mini]) mini = i - d;
//     if (A[i - d] > A[maxi]) maxi = i - d;
//     if (A[i] - A[mini] >= v) return new int[] {mini, i};
//     if (A[maxi] - A[i] >= v) return new int[] {maxi, i};
// }
// return new int[] { -1, -1};


// Runtime7 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory3.8 MB
// Beats
// 100%

// index >= index diff
// value >= value_diff
// min, max.
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        if index_difference == 0 && value_difference == 0 {
            return [0, 0].to_vec();
        }
        let mut mxi = 0;
        let mut mni = 0;
        for i in (index_difference as usize)..nums.len() {
            if (nums[i] - nums[mni]).abs() >= value_difference {
                return [mni as i32, i as i32].to_vec();
            }
            if (nums[i] - nums[mxi]).abs() >= value_difference {
                return [mxi as i32, i as i32].to_vec();
            }
            if i + 1 - index_difference as usize >= nums.len() {          //  [1] 0 1
                continue;
            }
            if nums[i + 1 - index_difference as usize] > nums[mxi] {
                mxi = i + 1 - index_difference as usize;
            }
            if nums[i + 1 - index_difference as usize] < nums[mni] {
                mni = i + 1 - index_difference as usize;
            }
        }

        return [-1, -1].to_vec();
    }
}


struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


