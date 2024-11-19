

// D D

// for (int a: A) {
//     res += ma;
//     ma = max(ma, a);
// }

// 角度完全不同。


// LC 上的 rust挺奇怪的。之前写的是 (stidx - i)  Example 1 是5， 没有编译错误，结果不正确。 就是 (3usize - 4usize) as i64 是可以计算的，得到 -1 i64 。 我本地试了下，编译失败了。
// 知道了， rust是 编译时校验，估计 LC上把 部分编译错误 忽略了。 所以 可以执行，而且得到 -1. 本地编译不了，不知道怎么降低 编译警告级别。 不过这个是 error 级别啊。
// ..知道了。 我是直接 (3usize - 4usize) 编译器估计尝试优化为 -1，但是 失败了。   而 (idx - i) 这种是运行时才知道的，所以 可以编译，只不过结果不正确
// attempt to compute `3_usize - 4_usize`, which would overflow
// 运行时，溢出就溢出，继续算，并且转为 i64 正好是 正确的 -1.

// Runtime
// 23ms
// Beats31.11%
// Analyze Complexity
// Memory
// 3.61MB
// Beats71.11%


// 升序
// (j-i)*nums[i] 就是 路径长度 * 起点权重。
// 如果 中间有一个点 权重大于 起点，那么 完全可以 跳到这个 中间点，这样 后半程的 路径长度 * 权重 是 更大的值

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let mut stidx = 0usize;
        let mut ans = 0i64;

        for i in 1..(nums.len() - 1) {
            if nums[i] > nums[stidx] {
                ans += (i - stidx) as i64 * nums[stidx] as i64;
                stidx = i;
            }
        }

        ans += (nums.len() - 1 - stidx) as i64 * nums[stidx] as i64;

        ans
    }
}




struct Solution {}

fn main()
{

    let a = 3usize;
    let b = 4usize;

    println!("{:?}", (a-b) as i32);

    // println!("ans: {:?}", Solution::());
}


