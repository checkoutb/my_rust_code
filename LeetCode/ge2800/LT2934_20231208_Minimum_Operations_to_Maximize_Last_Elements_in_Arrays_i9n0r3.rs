





// Runtime5 ms
// Beats
// 35.71%
// Memory2.1 MB
// Beats
// 35.71%

// sooooo, just try {nums1[sz1-1], nums2[sz2-1]} and {nums2[sz2-1], nums1[sz1-1]} ?

impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let sz1 = nums1.len();
        let t2 = Self::try_satify(&nums1, &nums2, nums1[sz1 - 1], nums2[sz1 - 1]);
        let mut t3 = Self::try_satify(&nums1, &nums2, nums2[sz1 - 1], nums1[sz1 - 1]);
        if t3 != -1 {
            t3 += 1;        // swap nums1[sz1-1], nums2[sz1-1]
        }
        if t2 == -1 || t3 == -1 {
            t2.max(t3)
        } else {
            t2.min(t3)
        }
    }

    // -1: not satify
    fn try_satify(nums1: &Vec<i32>, nums2: &Vec<i32>, mx1: i32, mx2: i32) -> i32 {
        let mut cnt = 0i32;

        for i in 0..(nums1.len() - 1) {
            if nums1[i] <= mx1 && nums2[i] <= mx2 {
                // ok, noop
            } else if nums1[i] <= mx2 && nums2[i] <= mx1{
                // swap
                cnt += 1;
            } else {
                return -1;
            }
        }
        cnt
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


