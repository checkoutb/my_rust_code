


// D

// In this problem, we have one more constrain of
// (vowels * consonants) % k == 0.
// Assume the length is l and vowels == consonants,
// so (l / 2) * (l / 2) % k == 0,
// so l * l % (k * 4) == 0.

// We only need to find out the smallest l satifying the above equation,
// and only need to consider the substring with length l, 2l, 3l ...
// In additional to the solution in "need to know" section,
// we count the index in bucket with different module of l.

// ..
// 假设 beautiful substr 的长度是 l， 那么 就可以得出 l*l%(k*5) == 0
// ..我好像没有学过  (l / 2) * (l / 2) % k == 0  ==> l * l % (k * 4) == 0
// 就是  a/4%k == 0  ==>  a%(k*4) == 0     应该是对的(肯定对的，毕竟AC了)， 但是 什么定理？
// 估计没有定理， 就是 a/4 能被 k 整除的话， a 肯定能被 4k 整除。  上下都乘以4
// a/4%k == 0  <==>  a/4 = k*x  <==>  a = k*x*4  <==>  a%(k*4) == 0 .. wo ke zhen shi ge tiancai a ...

// 需要找到 最小的substr的长度 l， 然后 就是 2l,3l
// 有没有 1.5l ？ 没有， 应该 l 是 最小的 substr，  所以 l 只可能是  sqrt(4k) 或 。。？ 不知道了

// l^2 % 4k == 0  =>  l^2 = 0,4k,8k,12k,16k...  find the min, sqrt(4k*x) % 1.0 == 0.0

// prefix sum 也很难。
// 首先要 按照 %l 进行分组，因为 最小的 substr 的长度 是 l，
// 然后 l 分组后， 要看 vowels 是否 等于 consonants， 就有了 第二维。  第二维就是保存 上次出现的 v-c的差， 本次再出现这个 差，说明 上次到本次 的 substr 中 v == c
// v == c (第二维) 满足 第一个条件，  按 %l 分组 (第一维) 满足了 第二个 条件。


// int n = s.length(), v = 0, l;
// for (l = 1; l * l % (k * 4); ++l);
// set<char> vowels = {'a', 'e', 'i', 'o', 'u'};
// vector<unordered_map<int, int>> seen(l);
// seen[l - 1][0] = 1;
// long long res = 0;
// for (int i = 0; i < n; i++) {
//     v += vowels.find(s[i]) != vowels.end() ? 1: -1;
//     res += seen[i % l][v]++;
// }
// return res;



// g


// hint 1
// For the given k find all the x integers such that x^2 % k == 0. 
// Notice, that there aren’t many such candidates.

// k=1时，是所有的 平方数， 最大是 2.5 * 10^5 的平方。 。。。 不知道后续。



// 感觉是 [i,j]可以的话， x<i时 [x,j] 也可以

// a==b , (a*b)%k==0 ,  (a^2)%k==0  除了a=sqrt(k)， a必须是 k 的倍数。 ，不， 6*6%4==0. 要么倍数，要么倍数*sqrt
// 没什么用啊。

// 683 / 684  tle           // ababab....abab..abab , k = 1

// 2947's code is tle. 678 / 684
// v == c ... vv-vc = vv-vc
// vv[i] - vv[j] == vc[i] - vc[j]  <=>  vv[i] - vc[i] = vv[j] - vc[j]
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let sz1 = s.len();
        let s = s.as_bytes();
        let mut vv = vec![0; sz1];      // <=idx, count of Vowel
        let mut vc = vec![0; sz1];      //                 Consonant
        if Self::is_vowel(s[0]) {
            vv[0] = 1;
        } else {
            vc[0] = 1;
        }
        for i in 1..sz1 {
            if Self::is_vowel(s[i]) {
                vv[i] = vv[i - 1] + 1;
                vc[i] = vc[i - 1];
            } else {
                vv[i] = vv[i - 1];
                vc[i] = vc[i - 1] + 1;
            }
        }

        use std::collections::HashMap;
        let mut map2: HashMap<i32, Vec<usize>> = HashMap::new();      // <i32, Vec<usize>>

        for i in 0..sz1 {
            let t5 = vv[i] - vc[i];
            if !map2.contains_key(&(t5)) {
                map2.insert(t5, Vec::new());
            }
            map2.get_mut(&t5).unwrap().push(i);
        }

        let mut t2;
        let mut t3;
        let mut ans = 0;
        for (key, v) in map2.iter() {
            for i in 0..v.len() {
                for j in (i + 1)..v.len() {
                    t2 = vv[v[j]] - vv[v[i]];
                    t3 = vc[v[j]] - vc[v[i]];
                    if (t2 * t3 % k) == 0 {
                        ans += 1;
                    }
                }
            }
            if *key == 0 {
                for i in 0..v.len() {
                    t2 = vv[v[i]];
                    t3 = vc[v[i]];
                    if (t2 * t3 % k) == 0 {
                        ans += 1;
                    }
                }
            }
        }
        ans

        // let mut ans = 0;
        // let mut en = 1usize;        // substr [st, en]
        // while en < sz1 {
        //     if vv[en] == vc[en] && (vv[en] * vv[en]) % k == 0 {
        //         ans += 1;
        //     }
        //     let mut prev_st = en % 2;
        //     let mut t2;
        //     let mut t3;
        //     while prev_st < en {
        //         t2 = vv[en] - vv[prev_st];
        //         t3 = vc[en] - vc[prev_st];
        //         if t2 == t3 && (t2 * t3) % k == 0 {
        //             ans += 1;
        //         }
        //         prev_st += 2;
        //     }
        //     en += 1;
        // }
        // ans
    }

    fn is_vowel(ch: u8) -> bool {
        ch == b'a' || ch == b'e' || ch == b'i' || ch == b'o' || ch == b'u'
    }
}





struct Solution {}

fn main()
{

    let s = "baeyh".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::beautiful_substrings(s, k));
}


