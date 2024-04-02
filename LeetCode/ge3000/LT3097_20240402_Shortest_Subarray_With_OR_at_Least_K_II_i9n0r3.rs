







// Runtime
// 42ms
// Beats42.11%of users with Rust
// Memory
// 4.12MB
// Beats36.84%of users with Rust

// 2 pointer
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut st = 0usize;
        let mut ans = i32::MAX;
        let mut t2 = 0i32;
        let mut vi = vec![0; 31];
        for i in 0..nums.len() {
            let mut n2 = nums[i];
            for j in 0..31 {
                if n2 & (1 << j) != 0 {
                    if vi[j] == 0 {
                        t2 |= (1 << j);
                    }
                    vi[j] += 1;
                }
            }

            while t2 >= k && st <= i {
                ans = ans.min((i - st + 1) as i32);
                n2 = nums[st];
                for j in 0..31 {
                    if n2 & (1 << j) != 0 {
                        vi[j] -= 1;
                        if vi[j] == 0 {
                            t2 = t2 ^ (1 << j);
                        }
                    }
                }
                st += 1;
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


