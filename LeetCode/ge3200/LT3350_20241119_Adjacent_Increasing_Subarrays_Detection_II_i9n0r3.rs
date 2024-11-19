

// D D

// pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
//     let mut prev = 1_000_000_000i32;
//     let mut prevLen = 0;
//     let mut start = 0;
//     let mut k = 0;
//     // concat slices~
//     let nums = [&nums[..], &[-1000]].concat();
//     for (i, &x) in nums.iter().enumerate() {
//         if (x <= prev) {
//             let len = i - start;
//             k = max(len/2, k);
//             k = max(min(len, prevLen), k);
//             // reset
//             start = i;
//             prevLen = len;
//         }
//         prev = x;
//     }
//     return k as i32;
// }
// ????????


// 。。 直接记录 当前 和 上一个 子数组。。




// Runtime
// 31ms
// Beats29.85%
// Analyze Complexity
// Memory
// 4.67MB
// Beats83.58%



// 2 个 相邻的 相同长度的 最大严格上升子数组
// 求 子数组最大长度

// 就是 正向，反向， 记录 以i为开始的 严格上升 子数组的最大长度， 以i为结尾的 严格上升 xxx

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut vi = vec![1; nums.len()];

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                vi[i] = vi[i - 1] + 1;
            }
        }

        let mut mxsfx = 1;
        let mut ans = 1;
        for i in (1..nums.len() - 1).rev() {
            ans = ans.max(mxsfx.min(vi[i]));
            if nums[i] < nums[i + 1] {
                mxsfx += 1;
            } else {
                mxsfx = 1;
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


