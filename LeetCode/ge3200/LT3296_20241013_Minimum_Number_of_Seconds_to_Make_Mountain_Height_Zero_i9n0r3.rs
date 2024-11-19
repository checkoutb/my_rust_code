

// D D

// 二分时间


// class Solution:
//     def minNumberOfSeconds(_, h, w):
//         return [*islice(merge(*map(accumulate, map(count, w, w))), h)][-1]
// ..我肯定没有看懂。


// 二分，一般会tle，需要缩短计算，要用到 sqrt的 。估计是 等差数列和 < (st+en)/2 ，而不是 直接 硬加 直到 超过 md



// Runtime
// 55ms
// Beats36.84%
// Analyze Complexity
// Memory
// 2.54MB
// Beats43.86%


// worker_time 是 reduce 1 的代价。 并不是 能 reduce 多少。
// 肯定是让 代价低的 多干活。唉~~~
// ..

// 好混乱。。

impl Solution {
    pub fn min_number_of_seconds(mut mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;

        let mut priq = BinaryHeap::new();

        worker_times.into_iter().for_each(|tm| priq.push((-tm as i64, (tm + tm) as i64, tm as i64)));

        let mut ans = 0i64;
        while mountain_height > 0 {
            mountain_height -= 1;

            let (a,b,c) = priq.pop().unwrap();

            // ans -= a as i64;
            ans = ans.max(-a as i64);

            priq.push((a - b, b + c, c));
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


