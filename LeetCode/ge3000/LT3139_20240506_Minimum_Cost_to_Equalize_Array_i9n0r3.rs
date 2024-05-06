





// 如果 cost1 * 2 <= cost2, 那么肯定是 全部 op 1
// >，每次都是op 2, 只有最后一次，如果 。。 example 1 就不行了。 还有 [123,123,5]
// 所以是， 全1 要多少cost，然后 尝试 2个 op 1 合并成 一个 op 2 ？

// 就是： 操作完， 最大值 还是 操作前的 最大值吗？ 11%的通过率，感觉 这个 最大值 会 变大。。但是想不出例子。 有的 [123,123,123,123,123,123, 1 2 2 ] op1:100000, op2:1

// 全op1, 达到 之前的最大值，肯定是 答案的上界

impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


