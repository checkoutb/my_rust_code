


// Runtime
// 16ms
// Beats100.00%
// Memory
// 2.42MB
// Beats100.00%



// /2  round up,  each ele at most once, total at most op1
// -k  [i]>=k,  each ele 1, total op2

// min sum

// sort,  /2 > k , then /2 , then -k   for -k is fixed: sum-k
// : 只有一个元素： k+2， 那么需要 -k 在 /2
// 如果 /2 >= k, 那么 /2, -k      。。 不知道会不会有什么很极限的情况。
//              否则 >k -> -k， /2   。。 这里不能/2 因为 -k后可能是一个 很小的值， 下一个元素比这个值大。
//              否则 /2     <k不能-k    。。所以 本行 和上一行的 /2 改成 放入 pri_q?


// k+k+1   5+5+1  11  /2  6   :1
//               6 /2 : 3

// ... -k 也不是乱减的。。 example 2.....

// 似乎无解啊， sz1 < 100，难道暴力？

// 难道预计算： ==k的直接 -k ？  应该是 [k, k+k-1) 的 -k， 因为它们只有这一种op。
//          .. 还要 先 sort

// [6,6,9], 5, 2, 2 : 4    ... 9/2-5=0, 6/2=3, 6-5=1.. 0+3+1

// hint1 : dp

// 对每个下标，尝试/2, 尝试-k， 尝试/2+-k
// dp1 dp2 最多 sz1  100*100
// 时间是 100*100*100

// [275,208,626], 553, 3, 2 -> 279      ...626 -k /2...

impl Solution {

    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let mut nums = nums;
        let mut op1 = op1 as usize;
        let mut op2 = op2 as usize;
        let sz1 = nums.len();

        nums.sort_by_key(|a| -a);

        let mut vvi = vec![vec![i32::MAX; op2 + 1]; op1 + 1];    // 已使用 i个 op1， j个op2 的 min sum
        vvi[0][0] = 0;

        for m in 0..sz1 {
            let mut vv2 = vec![vec![i32::MAX; op2 + 1]; op1 + 1];

            let t2 = nums[m];

            for i in 0..vvi.len() {
                for j in 0..vvi[0].len() {
                    if vvi[i][j] == i32::MAX {
                        break;
                    }
                    // 在 vvi[i][j] 的基础上
                    // /2 -k
                    if (t2 + 1) / 2 >= k && i + 1 <= op1 && j + 1 <= op2 {
                        vv2[i+1][j+1] = vv2[i+1][j+1].min(vvi[i][j] + ((t2 + 1) / 2 - k));
                    }
                    // -k /2
                    if t2 >= k && i + 1 <= op1 && j + 1 <= op2 {
                        vv2[i+1][j+1] = vv2[i+1][j+1].min(vvi[i][j] + (t2 - k + 1) / 2);
                    }

                    // /2
                    if t2 > 1 && i + 1 <= op1 {
                        vv2[i+1][j] = vv2[i+1][j].min(vvi[i][j] + (t2 + 1) / 2);
                    }

                    // -k
                    if t2 >= k && j + 1 <= op2 {
                        vv2[i][j + 1] = vv2[i][j + 1].min(vvi[i][j] + t2 - k);
                    }

                    // do nothing
                    vv2[i][j] = vv2[i][j].min(vvi[i][j] + t2);     // ..我应该保存 减去最多的值。。
                }
            }

            // for i in 0..vvi.len() {
            //     for j in 0..vvi[0].len() {
            //         if vvi[i][j] == i32::MAX {
            //             break;
            //         }
            //         print!("{}, ", vvi[i][j]);
            //     }
            //     println!();
            // }

            vvi = vv2
        }

        let mut ans = i32::MAX;
        for i in 0..vvi.len() {
            for j in 0..vvi[0].len() {
                if vvi[i][j] == i32::MAX {
                    break;
                }
                // print!("{}, ", vvi[i][j]);
                ans = ans.min(vvi[i][j]);
            }
            // println!("");
        }

        ans
    }

    pub fn min_array_sum_errrrr(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let mut nums = nums;
        let mut op1 = op1;
        let mut op2 = op2;
        let mut sum2 = nums.iter().sum();
        let mut vi = vec![];  // need /2

        nums.sort();
        for i in 0..nums.len() {
            if nums[i] >= k && nums[i] < k+k-1 {
                if op2 > 0 {
                    op2 -= 1;
                    nums[i] -= k;
                    sum2 -= k;
                } else {
                    break;
                }
            }
        }

        nums.sort_by_key(|a| -a);
        let mut t2 = 0;
        for i in 0..nums.len() {
            t2 = nums[i];
            if (t2+1) / 2 >= k {   // +1
                if op1 > 0 {
                    op1 -= 1;
                    sum2 -= t2 / 2;
                }
                if op2 > 0 {
                    op2 -= 1;
                    sum2 -= k;
                }
            } else {
                if t2 >= k {
                    if op2 > 0 {
                        op2 -= 1;
                        sum2 -= k;
                        t2 -= k;
                    }
                }
                vi.push(t2);
            }
        }

        if op1 > 0 {
            vi.sort_by_key(|a| -a);
            for i in 0..vi.len() {
                if op1 > 0 {
                    op1 -= 1;
                    sum2 -= vi[i] / 2;
                } else {
                    break;
                }
            }
        }

        sum2
    }
}



struct Solution {}

fn main()
{
    // let vi = [2,3,8,3,19].to_vec();
    // let k = 3;
    // let op1 = 1;
    // let op2 = 1;

    let vi = [275,208,626].to_vec();
    let k = 553;
    let op1 = 3;
    let op2 = 2;

    println!("ans: {:?}", Solution::min_array_sum(vi,k,op1,op2));
}


