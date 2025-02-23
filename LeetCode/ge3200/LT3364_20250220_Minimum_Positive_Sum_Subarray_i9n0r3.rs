


// D

// let mut min_sum = None;
// if sum > 0 && !min_sum.is_some_and(|s| s <= sum) {
//     min_sum = Some(sum);
// }





// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.28MB
// Beats75.00%


// size [l, r],  sum>0, min sum

// sz1 < 100, 似乎只能brute force?

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans = -1;
        let sz1 = nums.len();
        let mut vi = vec![0; sz1];
        let mut cnt = 0;
        let mut sum2 = 0;
        for i in 0..sz1 {
            sum2 += nums[i];
            cnt += 1;
            if cnt == l {
                vi[i] = sum2;
                sum2 -= nums[i + 1 - cnt as usize];
                cnt -= 1;
            }
        }
        let gap = (r - l) as usize;
        for i in (l - 1) as usize .. sz1 {
            sum2 = vi[i];
            for j in i..(i + gap + 1) {
                if (sum2 > 0) {
                    if ans == -1 {
                        ans = sum2;
                    } else {
                        ans = ans.min(sum2);
                    }
                }
                
                if j + 1 < sz1 {
                    sum2 += nums[j + 1];
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


