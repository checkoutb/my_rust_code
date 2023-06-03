








// Runtime71 ms
// Beats
// 95.24%
// Memory2.2 MB
// Beats
// 42.86%

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        
        use std::collections::HashMap;
        let mut map2: HashMap<i32, i32> = HashMap::new();

        for a in nums1.iter() {
            for b in nums2.iter() {
                *map2.entry((a + b)).or_insert(0) += 1;
            }
        }
        let mut ans: i32 = 0;
        for c in nums3.iter() {
            for d in nums4.iter() {
                ans += match map2.get(&(-c - d)) {
                    Some(val) => val,
                    None => &0
                }
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{


    //println!("ans: {:?}", Solution::());
}


