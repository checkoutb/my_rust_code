


// D D

// match (total - running).abs() {
//     0 => count += 2,
//     1 => count += 1,
//     _ => {}
// };



// Runtime
// 2ms
// Beats71.43%
// Analyze Complexity
// Memory
// 2.16MB
// Beats42.86%


// 起始于  值为0 的某个下标   选择方向
// 如果 [idx] == 0，则 按照选择的方向移动
// 如果 [idx] > 0，则 值-1，  反转方向，移动一步。

// 左侧和 == 右侧和， 2个方向 都 ok 
// 左侧和 与 右侧和 之差.abs == 1， 单个方向ok

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut sum2 = nums.iter().sum::<i32>();
        let mut got = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                let t2 = sum2 - got;
                if t2 == got {
                    ans += 2;
                } else if ((t2 - got) as i32).abs() == 1 {
                    ans += 1;
                } else if got > t2 {
                    break;
                }
            } else {
                got += nums[i];
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


