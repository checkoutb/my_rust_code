






// aaabbb  aabbb abbb aaabb aabb abb aaab aab


// error  g



// 应该是：
// 1. 遍历word，提取出可以 删除的 字符的个数的 序列。 比如 aaabb 就变成 [2,1]  就是 可以删除2个a，1个b
// 2. 组合， sum([2,1]) 取0，取1，取2，取3 . 要保证 至少 k 个

// 但是50万的 阶乘。。。而且还有 除法。。

// n取m: n!/(m!(n-m)!) 
// n 是固定的，即 sum([2,1])
// m 从0 -> n.min( word.size() - k )

// m=0:  n!/n! == 1
// m=1:  n!/(n-1)! == n == n / 1
// m=2:  n!/(2!*(n-2)!) == n*(n-1) / 2
// m=x:  n!/(x!*(n-x)!) == n*(n-1)*..*(n-x) / 1*2*3*..x  == A
// m=x+1        A * (n-x-1) / (x+1)
//
// 所以是可行的？

// no...   aaa => [2]    删除第二个a 和 删除第三个a 是相同的 original string: "aa"
//      有除法， 无法mod。

// 不管了，就硬算， tle 拉倒。  aaabb->[2,1] 这个优化可以使用。 不过 意义不大，来个 aabbaabb..也很长的。


// 至少 k 个字符
// 最多删除 sz - k 个字符， 删除的 必然是 连续的 的一部分。
// vi[i], 已经输入了 i 个 的 type序列
// 5*10^5 * 2000 == tle?!

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        const MOD : i32 = 1000000007i32;
        let mut vi = vec![0i32; k as usize];
        let mut cnt_ge_k = 0i32;
        // vi[1] = 1;  // word[0] must is origin string
        // let mut ans = 0i32;

        let word = word.into_bytes();

        let mut vcont = vec![];
        let mut cont = 0i32;
        for i in 1..word.len() {
            if word[i] == word[i - 1] {
                cont += 1;
            } else {
                if cont > 0 {
                    vcont.push(cont);
                }
                cont = 0;
            }
        }
        if cont > 0 {
            vcont.push(cont);
        }

        let tsum : i32 = vcont.iter().sum();  // can remove
        let got = word.len() - tsum as usize; // must type
        if got >= k as usize {
            cnt_ge_k = 1;
        } else {
            vi[got] = 1;
        }

        println!("{:?}, {}", vcont, got);

        for i in 0..vcont.len() {
            // delete 0,1,2..vcont[i]
            // 偏移，滑动窗口
            let mut v2 = vec![0i32; k as usize];
            // let mut t2 = 0i32;
            cont = vcont[i];
            for _a in 0..cont as usize {
                cnt_ge_k = (cnt_ge_k + cnt_ge_k) % MOD;
            }
            for j in got..vi.len() {
                if vi[j] > 0 {
                    for idx in 0..=cont as usize {
                        if j + idx < v2.len() {
                            v2[j + idx] = (v2[j + idx] + vi[j]) % MOD;
                        } else {
                            cnt_ge_k = (cnt_ge_k + vi[j]) % MOD;
                        }
                    }
                }
            }

            // for j in got..vi.len() {
            //     println!("{}, {}, {}",j - cont as usize - 1, t2, vi[j]);
            //     t2 = (t2 + vi[j] - if j as i32 - cont - 1 >= 0 { vi[j - cont as usize - 1] } else { 0 }) % MOD;
            //     v2[j] = (t2 + v2[j]) % MOD;                
            // }
            // while t2 > 0 {
            //     cnt_ge_k = (cnt_ge_k + t2) % MOD;
            //     println!("{},{},{},{}", vi.len(), cont, t2, vi.len() - cont as usize - 1);
            //     t2 = (t2 - vi[vi.len() - cont as usize - 1] + MOD) % MOD;
            //     cont -= 1;
            // }
            // let j = vi.len();
            // while j - 1 - (cont as usize) < vi.len() {
            //     cont -= 1;
            //     t2 = (t2 - vi[j - cont as usize] + MOD) % MOD;
            //     cnt_ge_k = (cnt_ge_k + t2) % MOD;
            // }
            vi = v2;

            println!("{}, {:?}, {}", i, vi, cnt_ge_k);

        }

        cnt_ge_k
    }
}



struct Solution {}

fn main()
{

    // let s = "aabbccdd".to_string();
    // let k = 7;

    // let s = "aabbccdd".to_string();
    // let k = 8;

    let s = "aaabbb".to_string();
    let k = 3;

    println!("ans: {:?}", Solution::possible_string_count(s, k));
}


