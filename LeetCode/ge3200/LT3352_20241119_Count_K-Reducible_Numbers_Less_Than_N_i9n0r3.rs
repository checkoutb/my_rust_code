

// D D

// def countKReducibleNumbers(self, s: str, k: int) -> int:
//         dp = [0] * 1000
//         for i in range(2, 1000):
//             dp[i] = dp[i.bit_count()] + 1
//         c1 = 0
//         n = len(s)
//         res = 0
//         mod = 10 ** 9 + 7
//         for i in range(n):
//             if s[i] == '1':
//                 for c2 in range(n - i):
//                     if c1 + c2 > 0 and dp[c1 + c2] + 1 <= k:
//                         res += comb(n - i - 1, c2) % mod
//                 c1 += 1
//         return res % mod
//
// py的整形无限大，可以 comb(800, 400)。
// 其他语言也可以 无限大，只不过需要自己实现。 
// 不知道 py的 comb里面是怎么做的，不可能真的 800! 然后 除以 400! 吧?



// Runtime
// 47ms
// Beats88.89%
// Analyze Complexity
// Memory
// 2.13MB
// Beats100.00%

// 有一点思路 -> 开始写代码 -> 无法用代码来完善思路。 
// 思考是跳跃的，但是代码必须 有输入，有输出。  无法构成一个 完整的 证明
// 无法证明是对的，无法证明是错的， (无限)重复证明过程，但依然无法证明。
// 代码中一点点的 错误(笔误)，导致结果始终不正确，难以排查。
// 边界条件，是否有其他代码已经计算过这个边界条件。


// 1ge 1   1000..00  -> 1
// 2ge 1   1001000 -> 10 -> 1
// 3      xx -> 11 -> 10 -> 1
// 4  xx- > 100 -> 1

// n: 10101010

// < n

// 只想到 每个 1 bit 计算一下 ( 比如  10010， 第一个1 bit，计算 1xxx 有多少个) 。 最多做一些 memo。

// 还有 800 C 400， 这个没有办法用 MOD 啊。
// 前几天 遇到过，忘记了， 好像可能是 矩阵乘法， 也可能不是。完全忘记了。

// 所以还是 ： 构建全部， 然后判断 哪些 可以
//     就是，把 1-n 所有值 分类， 按 bit 1 的个数分类。 那么 最多 800个分类。  
//     然后 对 1-800 直接计算 能不能 k次 下降到 1.      实际上 1-800 可以用 上一步的 1-n 的结果来直接 算出 bit 1 的个数。  用不了
// 应该是这种方法。


impl Solution {
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        let mut cnt1 = 0i32;
        let s = s.into_bytes();
        const B_one: u8 = b'1';
        const MOD: i32 = 1000000007;

        s.iter().for_each(|e| cnt1 += if *e == B_one { 1 } else { 0 });

        let mut vi = vec![0; 801];

        // let mut v2 = vec![0; 2];   // first char is 0 or 1
        let mut vi0 = vec![0; 801];   // first ch is 0, 1's count is idx
        let mut vi1 = vec![0; 801];

        vi0[0] = 1;

        let mut cnt2 = 0;  // scanned, so can remove
        for i in (1..s.len()).rev() {  // [0]
            if s[i] == B_one {
                // xxxxx1xxxxx
                //      i
                // xxxxx0yyyyy
                cnt2 += 1;
                for j in 0..vi0.len() {
                    if (j + (cnt1 - cnt2) as usize) < vi.len() {
                        vi[j + (cnt1 - cnt2) as usize] += vi0[j];  // cn1-cnt2 is prefix's 1's count
                        vi[j + (cnt1 - cnt2) as usize] %= MOD;
                        vi[j + (cnt1 - cnt2) as usize] += vi1[j];
                        vi[j + (cnt1 - cnt2) as usize] %= MOD;
                    }
                }

                // for j in 0..4 {
                //     println!("   {}", vi[j]);
                // }
            }

            ///////// add1, add0

            let mut vi00 = vec![0; vi0.len()];
            let mut vi11 = vec![0; vi1.len()];
            for j in 0..vi0.len() {
                // add 0, vi0 += vi1,  vi1 = {0}
                // add 1, vi1 = vi1+1, vi1 = vi1 + vi0-1

                vi00[j] = (vi0[j] + vi1[j]) % MOD; // add 0
                vi11[j] = if j == 0 { 0 } else { (vi0[j - 1] + vi1[j - 1]) % MOD };
            }
            vi0 = vi00;
            vi1 = vi11;

        }

        // [0] == 1
        for i in 1..vi0.len() {
            vi[i] += (vi0[i] + vi1[i]) % MOD;
            vi[i] %= MOD;
        }

        // println!("{:?}\n{:?}\n{:?}", vi0, vi1, vi);
        // for i in 0..4 {
        //     println!("{}, {}, {}", vi0[i], vi1[i], vi[i]);
        // }

        let mut mnstep = vec![0i32; 801];
        // mnstep[1] = 0;
        for i in 2..mnstep.len() {
            let mut t2 = i;
            let mut cnt4 = 0usize;
            while t2 > 0 {
                cnt4 += 1;
                t2 &= t2 - 1;
            }
            mnstep[i] = mnstep[cnt4] + 1;
        }

        // for i in 0..4 {
        //     println!("{}", mnstep[i]);
        // }

        let mut ans = 0;
        for i in 0..vi.len() {
            if vi[i] != 0 && mnstep[i] < k {  // <
                ans = (ans + vi[i]) % MOD;
            }
        }
        ans
    }

    // fn cala1(mut a: i32) -> i32 {
    //     if a == 0 {
    //         return 0;
    //     }
    //     while 
    // }
}



struct Solution {}

fn main()
{

    // let s = "111".to_string();
    // let k = 1;


    // 0: 1      001
    // 1: 2,4    010  100
    // 2: 3,5,6  011  101  110
    // 3: 7     7-3-2-1  111
    // let s = "1000".to_string();
    // let k = 2;



    // // 1 10 11 100
    let s = "101".to_string();  // 4 
    let k = 3;

    println!("ans: {:?}", Solution::count_k_reducible_numbers(s, k));
}


