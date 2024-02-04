


// for &n in &nums {




// Runtime2 ms
// Beats
// 40%
// Memory2.2 MB
// Beats
// 40%

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        // let mut vi : Vec<i32> = Vec::with_capacity(31);  // capacity, not size/length
        let mut vi = vec![0; 31];
        for n in nums.iter() {
            for i in 0..31 {
                if (n & (1 << i)) != 0 {
                    vi[i] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..31 {
            if vi[i] >= k {
                ans |= 1 << i;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [7,12,9,8,9,15].to_vec();
    let k = 4;

    println!("ans: {:?}", Solution::find_k_or(vi, k));
}


