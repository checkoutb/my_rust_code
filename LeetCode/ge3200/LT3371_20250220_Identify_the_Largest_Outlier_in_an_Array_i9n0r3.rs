



// Runtime
// 67ms
// Beats23.08%
// Memory
// 3.09MB
// Beats100.00%




// 有负数。
// 移除 outlier 后，剩下的元素可以分为 1个集合 + 一个单个元素， sum相同。
//
// kao, (sum_all - outlier) - not_special = not_special

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        // let mut vb = vec![false; 2001];   // is exists?
        let mut vcnt = vec![0; 2001];
        nums.sort_by_key(|a| -a);
        let mut sum2 = 0;
        for i in 0..nums.len() {
            sum2 += nums[i];
            vcnt[(nums[i] + 1000) as usize] += 1;
        }

        for i in 0..nums.len() {
            let mut t2 = sum2 - nums[i];
            if t2 % 2 != 0 {    //  -1 % 2 == -1   != 1....
                continue;
            }
            // println!("{}, {}, {}", (t2 / 2), (t2 >> 1), t2 % 2);
            t2 = t2 / 2;
            if t2 >= -1000 && t2 <= 1000 && vcnt[(t2 + 1000) as usize] != 0 {
                if t2 == nums[i] {
                    if vcnt[(t2 + 1000) as usize] > 1 {
                        return nums[i];
                    }
                } else {
                    return nums[i];
                }
            }
        }

        unreachable!();
    }
}


struct Solution {}

fn main()
{

    // let mut a = -202;
    // println!("{}", (a>>1));

    // println!("{}, {}", (a / 2), (a>>1));

    let vi = [-845,-56,797,-198,878,469,-762,655,-580].to_vec();

    println!("ans: {:?}", Solution::get_largest_outlier(vi));
}


