












// Runtime
// 30ms
// Beats87.10%
// Memory
// 8.07MB
// Beats19.35%

impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {

        use std::collections::BinaryHeap;
        let mut priq = BinaryHeap::new();

        
        let sz1 = nums1.len();
        let mut vp = vec![(0, 0); sz1];

        for i in 0..sz1 {
            vp[i].0 = nums1[i];
            vp[i].1 = i;
        }

        let mut vans = vec![0i64; sz1];
        vp.sort();
        let mut st = 0;
        let mut sum2 = 0i64;
        for i in 0..sz1 {
            while vp[st].0 < vp[i].0 {
                priq.push(-nums2[vp[st].1]);
                sum2 += nums2[vp[st].1] as i64;
                st += 1;
            }
            while priq.len() > k as usize {
                sum2 += priq.pop().unwrap() as i64;
            }
            vans[vp[i].1] = sum2;
        }

        vans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


