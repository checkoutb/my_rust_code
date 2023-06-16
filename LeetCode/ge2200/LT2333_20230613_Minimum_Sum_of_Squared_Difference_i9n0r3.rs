







// Runtime170 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory4.3 MB
// Beats
// 100%

/*
a^2 + b^2 - 2ab

(a-b-1)^2
((a-1)-b)^2
(a-1)^2 + b^2 - 2(a-1)b

a^2 + 1 - 2a + b^2 - 2ab + 2b
a^2 + b^2 - 2ab
    +1 -2a + 2b

a > b

a-1 === b+1

2*min + 1 - 2*max

(8-3)^2 = 25
(7-3) = 16
(8-4) = 16

2*3 + 1 - 2*8 = 7-16 = -9 = 
25 + (-9) = 16

9

-7

2max - 2min - 1   substract

priq

16, 9, 4, 1, 0
  7  5   3  1

binary search ? [0, mx.sqrt]

(mx - x) * cnt <= k
x >= mx - k/cnt

*/

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        // use std::num;

        let mut vi: Vec<i64> = vec![0; nums1.len() + 1];
        let mut ans: i64 = 0;
        vi[nums1.len()] = 0;
        for i in 0..nums1.len() {
            vi[i] = (nums1[i] - nums2[i]).abs() as i64;
            ans += vi[i] * vi[i];
            // println!("1 {}", ans);
            // vi[i] = (2 * nums1[i].max(nums2[i]) - 2 * nums1[i].min(nums2[i]) - 1) as i64;
            vi[i] = vi[i] * vi[i];
        }
        vi.sort();
        vi.reverse();
        let mut mx = vi[0];
        let mut cnt = 1;
        let mut k = (k1 + k2) as i64;
        if k == 0 {
            return ans;
        }
        println!("{}", ans);
        println!("{:?}", vi);
        for i in 1..vi.len() {
            if mx > vi[i] {
                // let t2 = cnt * (num::sqrt(mx as float64) - num::sqrt(mn));
                let t2 = cnt * ((mx as f64).sqrt() as i64 - (vi[i] as f64).sqrt() as i64);
                if t2 >= k {
                    let t3 = (mx as f64).sqrt() as i64 - k / cnt;
                    ans -= mx * cnt - (t3 * t3) * cnt;
                    ans -= t3 * t3 * (k % cnt) - (t3 - 1) * (t3 - 1) * (k % cnt);
                    break;
                } else {
                    // ans -= (mx + vi[i]) * (t2) / 2;
                    // let mut t3 = mx;
                    // while t3 > vi[i] {
                    //     ans -= t3 * 
                    // }
                    ans -= mx * cnt;
                    ans += vi[i] * cnt;
                    k -= t2;
                }
                // let t2 = cnt * (mx - cnt) / 2;
                // if t2 >= k {
                //     ans -= (mx + (mx - (k / cnt) * 2)) * (k / cnt) / 2;
                //     ans -= (mx - k / cnt) * (k % cnt);
                //     break;
                // } else {
                //     ans -= (mx + vi[i]) * (t2) / 2;
                //     k -= t2;
                // }
                mx = vi[i];
            }
            println!("i: {}, k: {}, mx: {}, cnt: {}", i, k, mx, cnt);
            cnt += 1;
        }
        ans


        // use std::collections::BinaryHeap;

        // let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
        // let mut ans = 0;
        // for i in 0..nums1.len() {
        //     ans += (nums1[i] - nums2[i]) * (nums1[i] - nums2[i]);
        //     if nums1[i] > nums2[i] {
        //         heap.push((2 * nums1[i] - 2 * nums2[i] - 1, nums1[i], nums2[i]));
        //     } else if nums1[i] < nums2[i] {
        //         heap.push((2 * nums2[i] - 2 * nums1[i] - 1, nums2[i], nums1[i]));
        //     }
        // }
        
        // let mut k = k1 + k2;
        // let mut mx = -1;
        // let mut cnt = -1;
        // while !heap.is_empty() && k > 0 {

        // }
    }
}




struct Solution {}

fn main()
{
    // let vi = [1,2,3,4].to_vec();
    // let v2 = [2,10,20,19].to_vec();
    // let k1 = 0;
    // let k2 = 0;

    let vi = [1,4,10,12].to_vec();
    let v2 = [5,8,6,9].to_vec();
    let k1 = 1;
    let k2 = 1;

    // let vi = [1,4,10,12].to_vec();
    // let v2 = [5,8,6,9].to_vec();
    // let k1 = 10;
    // let k2 = 5;

    println!("ans: {:?}", Solution::min_sum_square_diff(vi, v2, k1, k2));
}


