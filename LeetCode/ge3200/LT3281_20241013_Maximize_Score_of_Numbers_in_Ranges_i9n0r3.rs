

// pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
//     fn check(start: &[i64], d: i64, mid: i64) -> bool {
//     }
// }
// 这个学名叫什么？ 闭包？ lambda？ 很少见。


// Runtime
// 55ms
// Beats30.23%
// Analyze Complexity
// Memory
// 4.29MB
// Beats55.81%



// [100000,100000] 0
// [100000,100000] 100000

// [10亿，10亿] 10亿，  本地是 溢出了， 但是 leetcode上是返回了 20亿零1


// [1,5,6,7,990], 10     1 250 500 750 1000
// [1]
// [1,15]   
// [1,15,16]   [1,8,16]   [1,6,11,17,1000]
//
// 似乎有一种方法，但是不知道对不对，没有任何根据， 从后往前， ([idx] + d - [0]) / idx 。
// 似乎是正确的，但是 没有办法证明
// 每次都假设里面是最优分布，即每个元素i可以取 [0] + ([idx]+d-[0])/idx * i
// 。不行的，[1,15,16],1  这种就是反例，无法计算出2的。
//
// ..max。。不会二分吧。 看起来可以二分的。。 确实，记得以前注意到过，max，min之类的都可以二分。..
// 不是通常的二分。。 普通的二分。
impl Solution {
    pub fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
        start.sort();
        let enmx = start[start.len() - 1] + d;
        let mut st = 0; // not 1, [10000,10000],0
        let mut en = enmx / (start.len() as i32 - 1) + 1;
        let mut ans = -1;
        while st <= en {
            let md = st + (en - st) / 2;        // step
            let mut flg = 0;
            // let mut t2 = start[0] + md;   // will overflow?
            let mut t2: i64 = start[0] as i64 + md as i64;

            for i in 1..start.len() {
                if t2 < start[i] as i64 {
                    // ok
                    t2 = start[i] as i64 + md as i64;
                } else if t2 > start[i] as i64 + d as i64 {
                    // error
                    flg = 1;
                    break;
                } else {
                    // ok
                    if i != start.len() - 1 {
                        t2 += md as i64;  // overflow
                    }
                }
            }

            if flg == 0 {
                ans = md;
                st = md + 1;
            } else if flg == 1 {

                en = md - 1;
            } else {
                // never
            }

        }
        ans
    }
}



struct Solution {}

fn main()
{

    let vi = [1000000000,1000000000].to_vec();
    let k = 1000000000;

    println!("ans: {:?}", Solution::max_possible_score(vi, k));
}


