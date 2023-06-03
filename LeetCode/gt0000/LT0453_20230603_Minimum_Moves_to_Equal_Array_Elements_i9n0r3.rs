



// D D

// sum + m * (n - 1) = x * n
// x = minNum + m
// sum - minNum * n = m


// 感觉应该是数据归纳法

// 假设 n-1 个 都相同， 1个 远大于 其他的值。 那么 操作次数就是 大于的差值
// 在上面的基础上， n-1个中 选一个 + 1, 那么 操作次数就是 上面的ans + 1
// 在上面的基础上， n-1个中选一个 + 1， 上面的 ans + 1


// Runtime7 ms
// Beats
// 72.73%
// Memory2.3 MB
// Beats
// 9.9%
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut vi = nums;
        vi.sort();
        let mut ans: i32 = 0;
        for (_i, val) in vi.iter().enumerate() {
            ans += val - vi[0];
        }
        return ans;
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


