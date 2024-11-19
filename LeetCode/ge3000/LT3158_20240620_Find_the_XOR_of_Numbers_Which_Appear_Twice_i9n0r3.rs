







// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.21MB
// Beats15.38%
impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut vi = vec![false; 51];
        let mut ans = 0;
        for n in nums.into_iter() {
            if vi[n as usize] {
                ans ^= n;
            } else {
                vi[n as usize] = true;
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


