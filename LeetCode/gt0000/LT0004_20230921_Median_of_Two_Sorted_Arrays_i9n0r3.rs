

// D D


// public double findMedianSortedArrays(int[] A, int[] B) {
//     int m = A.length, n = B.length;
//     int l = (m + n + 1) / 2;
//     int r = (m + n + 2) / 2;
//     return (getkth(A, 0, B, 0, l) + getkth(A, 0, B, 0, r)) / 2.0;
// }

// public double getkth(int[] A, int aStart, int[] B, int bStart, int k) {
// if (aStart > A.length - 1) return B[bStart + k - 1];            
// if (bStart > B.length - 1) return A[aStart + k - 1];                
// if (k == 1) return Math.min(A[aStart], B[bStart]);

// int aMid = Integer.MAX_VALUE, bMid = Integer.MAX_VALUE;
// if (aStart + k/2 - 1 < A.length) aMid = A[aStart + k/2 - 1]; 
// if (bStart + k/2 - 1 < B.length) bMid = B[bStart + k/2 - 1];        

// if (aMid < bMid) 
//     return getkth(A, aStart + k/2, B, bStart,       k - k/2);// Check: aRight + bLeft 
// else 
//     return getkth(A, aStart,       B, bStart + k/2, k - k/2);// Check: bRight + aLeft
// }



// Runtime0 ms
// Beats
// 100%
// Memory2 MB
// Beats
// 85.9%

// log(10^7)*(log(n)+log(m))
// m+n < 2000
// max - min = 10^7
// 4 2
// 5 2
// 3 1
// 2 1
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut st1 = 0 as usize;
        let mut st2 = st1;
        let mut t2 = st1;
        if (nums1.len() + nums2.len()) % 2 == 0 {
            t2 = 1;
        }
        while st1 + st2 < (nums1.len() + nums2.len()) / 2 - t2 {
            if st1 == nums1.len() {
                st2 += 1;
            } else if st2 == nums2.len() {
                st1 += 1;
            } else {
                if nums1[st1] < nums2[st2] {
                    st1 += 1;
                } else {
                    st2 += 1;
                }
            }
        }
        println!("{},{}", st1, st2);
        if st2 == nums2.len() {
            if t2 == 1 {
                return ((nums1[st1] + nums1[st1 + 1]) as f64) / 2.0;
            } else {
                return nums1[st1] as f64;
            }
        } else if st1 == nums1.len() {
            if t2 == 1 {
                return ((nums2[st2] + nums2[st2 + 1]) as f64) / 2.0;
            } else {
                return nums2[st2] as f64;
            }
        } else {
            if t2 == 1 {
                return ((nums1[st1] + nums2[st2])
                    .min(nums1[st1] + (if (st1 + 1) < nums1.len() {nums1[st1 + 1]} else {123123123}))
                    .min(nums2[st2] + (if (st2 + 1) < nums2.len() {nums2[st2 + 1]} else {123123123})) as f64) / 2.0;
            } else {
                return nums1[st1].min(nums2[st2]) as f64;
            }
        }
    }
}





struct Solution {}

fn main()
{

    let vi = [1,2].to_vec();
    let v2 = [-1,3].to_vec();

    println!("ans: {:?}", Solution::find_median_sorted_arrays(vi, v2));
}
