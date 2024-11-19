





// error

// g



/*

source, pattern, idx

dp[i][j] = k
[source idx][pattern idx] = max num of op / max remove

source idx == for (xx)

dp[j] = k
for (i 0..source.len) {

    if i in targetIndex {
        // remove
        // 不变。

// 第一个到达 pattern[last] 的 就是 操作最多的？ 不， 第一个到达的可能是前面都不删除的

// vvp?  vector<vector<pair<int, int>>>
// 第一维 26个 字母，代表 需要对比的 pattern的 字符
// 第二维 是变长数组 里面保存 pair<pattern 下标，已执行的最大操作>

初始 全空，除了 [pattern[0]].push(make_pair(0, 0));

for i 0..source.len {
    if not delete {

        那么将 vvp[source[i]]  这里的 pattern idx 都 往后一位。  
        向后后，这个 pair 就去 对应的 vvp[ch]了， 
        当然， 下一个可能依然是本ch，所以需要 new 一个 vp 然后 swap

    } else {

        删除：
        。。。什么都不用做。 不，这里的 pair.second 得 ++


        不删除
        不删除 和 上面的分支 一样。


        要把上面的  删除，不删除  合并起来。

    }

    上面的 if else，
    vector 也可以 pop_front。
    如果可以删除，那么 for 原有的 vvp[source[i]] ， second++。
    如果不能删除，那么 for 原有的 vvp[source[i]]  pop .
    并且 for 的时候 都需要 处理： pattern.idx ++ 

    所以应该是， for vvp[source[i]] {
        处理 pattern.idx++

        if can delete {
            vvp[source[i]][j].second++
        } else {
            vvp[source[i]].pop_front
        }
    }


}





        // not remove
        // 同下，贪心，需要这个字符的 后移一位
    } else {
        // 贪心，把 需要 source[i] 这个字符的， 全部后移一位。
    }

    // for (j 0..pattern.len) {
    //     if s[i] == p[j] {
    //         // remove or not remove
    //         // remove:
    //         dp[j+1] = max(dp[j]+1, dp[j+1])

    //         // not remove
    //         dp[j] = dp[j];
    //     } else {
    //         dp[j] = dp[j];
    //     }
    // }

}

*/


// no pop_front ... c++ also no

impl Solution {
    pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
        
        // use std::collections::VecDeque;

        // let mut vvp = vec![VecDeque::new(); 26];

        let mut vvp = vec![vec![]; 26];
        let au8 : u8 = b'a';

        let mut target_indices = target_indices;
        target_indices.push(source.len() as i32);

        let (sarr, parr) = (source.into_bytes(), pattern.into_bytes());
        vvp[(parr[0] - au8) as usize].push((0, 0));
        let mut ans = 0;

        let mut tiidx = 0usize;
        for i in 0..sarr.len() {








// 这里错了，需要增加 for (0..26)








            let vvpi = (sarr[i] - au8) as usize;
            let vdsz = vvp[vvpi].len();
            let mut vp = vec![];
            for j in 0..vdsz {
                if i == target_indices[tiidx] as usize {
                    vp.push((vvp[vvpi][j].0, vvp[vvpi][j].1 + 1));
                }
                let (mut pidx, mxcnt) = vvp[vvpi][j];
                if sarr[i] == parr[pidx] {
                    pidx += 1;

                    if pidx as usize == parr.len() {

                        println!("{:?}, {:?}, {:?}", mxcnt, tiidx, i);

                        ans = ans.max(mxcnt + (target_indices.len() - tiidx - 1) as i32 
                        - if i == target_indices[tiidx] as usize {0} else {0});
                    } else {
                        if parr[pidx] == parr[pidx - 1] {
                            vp.push((pidx, mxcnt));
                        } else {
                            vvp[(parr[pidx] - au8) as usize].push((pidx, mxcnt));
                        }
                    }
                } else {
                    vp.push((pidx, mxcnt));
                }
            }

            vvp[(sarr[i] - au8) as usize] = vp;

            if i == target_indices[tiidx] as usize {
                tiidx += 1;
            }
        }
        ans
    }
}


// 所以应该是， for vvp[source[i]] {
//     处理 pattern.idx++

//     if can delete {
//         vvp[source[i]][j].second++
//     } else {
//         vvp[source[i]].pop_front
//     }
// }


struct Solution {}

fn main()
{

    // let s = "abbaa".to_string();
    // let p = "aba".to_string();
    // let vi = [0,1,2].to_vec();

    let s = "bcda".to_string();
    let p = "d".to_string();
    let vi = [0,3].to_vec();

    println!("ans: {:?}", Solution::max_removals(s,p,vi));
}


