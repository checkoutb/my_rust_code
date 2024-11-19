



// while let Some((mask, ans_prefix)) = stack.pop() {


    // if (a % 2 == 0) {
    //     res.push_back(-1);
    // } else {
    //     res.push_back(a - ((a + 1) & (-a - 1)) / 2);
    // }
//
// a+1,  -(a+1)
// a+1 就是把末尾的连续1 全遍历0， 连续1前面的0 变成1
// -(a+1), 按位取反再加1，  
// 比如  11001111
// a+1  11010000
// -(a+1)   00101111 + 1   00110000
// &  00010000
// /2 00001000   获得了 末尾连续1 的最高位的1。

// a & -a is a trick to get the rightmost 1





// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.20MB
// Beats53.33%


// tail all 1, first 1
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut vi = vec![-1; nums.len()];
        for (i, n) in nums.into_iter().enumerate() {
            if n % 2 != 0 {
                for j in 1..31 {
                    if n & (1 << j) == 0 {
                        vi[i] = n ^ (1 << (j - 1));
                        break;
                    }
                }
            }
        }
        vi
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


