



// hint   sparse table


// g


// 差分 + pfx + 二分？
// 差不了一点。。
// 不过有思路了，  从后往前， 计算下。 因为后面是非降的， 不过感觉，要 双指针？
// 好像也不太对。。 就是 缓存中放什么?
// 借助stack，   memo放 以[i] 为起点的 最长的 subarr的 长度。
// stack 不行。 好像行。 ..... 不行。。需要记录的数据太多了。

// 又想到一个： 双指针， 。。。


impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        if nums.len() == 1 {
            return 1i64;
        }
        let mut ans = 0i64;
        // let mut st = 0;
        let mut nxt_en = 0;
        
        for i in 0..nums.len() {
            if i > 0 {
                // minus
            }
        }

    }
}


impl Solution {
    // pub fn count_non_decreasing_subarrays__EEEE(nums: Vec<i32>, k: i32) -> i64 {
        
    //     if nums.len() == 1 {
    //         return 1i64;
    //     }

    //     let mut vi = vec![0; nums.len()];   // [idx, end] 是非降。。
    //     let mut stk = vec![];
    //     let mut v2 = vec![0; nums.len()];   // 下一个大于它的 距离。 stack的pop次数

    //     vi[vi.len() - 1] = 1;
    //     let mut ans = 1i64;
    //     stk.push(vi[vi.len() - 1]);

    //     let mut idx = nums.len() - 2;
        
    //     while idx >= 0 {
    //         let t2 = nums[idx];

    //         if t2 <= nums[idx + 1] {
    //             vi[idx] = vi[idx + 1] + 1;
    //             v2[idx] = 1;
    //         } else {
    //             let mut cnt = 0;
    //             while !stk.is_empty() && stk[i] < t2 {

    //             }
    //         }

    //         stk.push(t2);
    //         ans += vi[idx] as i64;
    //         if idx == 0 {
    //             break;
    //         }
    //         idx -= 1;
    //     }


    // }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


