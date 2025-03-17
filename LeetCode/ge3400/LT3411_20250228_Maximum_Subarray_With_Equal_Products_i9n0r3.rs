


// D D

// .... brute force


// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.27MB
// Beats96.77%



// ....   [2,6]   [4,6]

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut ans = 2;
        for i in 0..nums.len() {
        // for i in 0..=0 {
            // if i > 0 && nums[i] == nums[i - 1] {
            //     ans = ans.max(2);
            // }
            // if i > 0 && (nums[i] % nums[i - 1] == 0 || nums[i - 1] % nums[i] == 0) {
            //     ans = ans.max(2);
            // }
            // if i > 0 {

            // }
            let mut vcnt = vec![0; 10];
            for j in i .. nums.len() {
                let mut t2 = nums[j];
                // 2 3 5 7
                if t2 % 2 == 0 {
                    if vcnt[2] != 0 {
                        break;
                    }
                    vcnt[2] = 1;
                }
                if t2 % 3 == 0 {
                    if vcnt[3] != 0 {
                        break;
                    }
                    vcnt[3] = 1;
                }
                if (t2 % 5 == 0) {
                    if vcnt[5] != 0 {
                        break;
                    }
                    vcnt[5] = 1;
                }
                if (t2 % 7 == 0) {
                    if vcnt[7] != 0 {
                        break;
                    }
                    vcnt[7] = 1;
                }
                // println!("{}...{:?}", t2, vcnt);
                ans = ans.max(j - i + 1);
            }
        }
        ans as i32
    }
}


struct Solution {}

fn main()
{

    let vi = [3,1,6].to_vec();

    println!("ans: {:?}", Solution::max_length(vi));
}


