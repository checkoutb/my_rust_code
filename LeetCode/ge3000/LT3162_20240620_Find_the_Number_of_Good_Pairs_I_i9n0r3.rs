


// D D

// nums1.into_iter().map(|num| nums2.iter().copied().filter(|&x| num % (x * k) == 0).count()).sum::<usize>() as _




// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.12MB
// Beats19.42%
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let ans = 0;

        for n1 in nums1.into_iter() {
            for n2 in nums2.iter() {
                if n1 % (n2 * k) == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


