

// use std::collections::BinaryHeap;
// impl Solution {
//     pub fn max_sum(mut grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
//         let (m, n) = (grid.len(), grid[0].len());
//         let mut candidates = BinaryHeap::new();
//         for i in 0..m {
//             grid[i].sort();
//             for j in 0..limits[i] {
//                 candidates.push(grid[i][n - 1 - j as usize]);
//             }
//         }

//         (0..k)
//             .fold(0_i64, |pre, _| pre + candidates.pop().unwrap() as i64)
//     }
// }


// ...... 我的代码应该是错的，但是AC了。。
// 我的代码是 每行 取 不超过 limit 的 前x个  能组成的 最大sum
// AC后 看 其他人的代码， 和我的不一样。。 看了solution， 最后返过来看题目，才 发现。

// ................ 代码没错， 因为 我sort了。。。。 我忘记我 sort 了。。。


// Runtime
// 70ms
// Beats28.21%
// Memory
// 4.89MB
// Beats97.44%



impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        
        use std::collections::BinaryHeap;
        
        let mut priq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut grid = grid;
        for i in 0..grid.len() {
            grid[i].sort_by_key(|v| -v);
            if limits[i] > 0 {
                priq.push((grid[i][0], i, 1));
            }
        }

        let mut k = k;
        let mut ans = 0;
        while k > 0 {
            k -= 1;

            if let Some((v, i, j)) = priq.pop() {
                ans += v as i64;
                if (j as i32) < limits[i] {
                    priq.push((grid[i][j], i, j + 1));
                }
            } else {
                break;
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


