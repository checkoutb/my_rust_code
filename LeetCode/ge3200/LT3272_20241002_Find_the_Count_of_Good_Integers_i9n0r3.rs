


// D

// def countGoodIntegers(self, n: int, k: int) -> int:
//     n2 = (n + 1) // 2
//     res = 0
//     seen = set()
//     for v in range(10 ** (n2 - 1), 10 ** n2):
//         vv = str(v) + str(v)[::-1][n % 2:]
//         key = ''.join(sorted(vv))
//         if int(vv) % k == 0 and key not in seen:
//             seen.add(key)
//             count = Counter(vv)
//             x = (n - count['0']) * factorial(n - 1)
//             for i,c in count.items():
//                 x //= factorial(c)
//             res += x
//     return res
// 唉，差一点点，但是就是永远。
// 或许差的还很多吧。




// g  基础就错了， 不能 for(i+=k) 的 ， n4， k7 的情况下，只有 7700，7777 及其变种。 答案是 76个， 我想不出来。
// cnt_rearrange 也有问题，没有补前缀0.  7 只按7算， 没有按照 07算。




// palindrome,  所以只需要 遍历 前一半，所以 10^5， 应该不会tle
// /k 。 
// /2 就是偶数。   /8直接构建，所以 /2 直接 除以2就可以了，不需要构建。
// /3 就是 各个数字加起来能被3整除  。。 也可以直接 除以3吧。 毕竟可以和 /8 一样 for ( i+=3 )   。。 n是奇数  不能直接 除以3.    还是 for 构建，不然分支太多了。 。。
// /4 。 不知道有什么特殊的， 4 8 12 16 20 24 28 32 36。 所以是  偶数+480， 奇数+26 ？
// /5 0 or 5   。。。 不能以0结尾，任何数字 都不能以0结尾， 不然 有 前缀0
// /6   6 12 18 24 30 36 .不能和4一样， 因为 100%4是0，所以只需要看 最后2位，  6不行。 1+0 是无法 被 6整除的， 因为 1+0 无法被3整除，自然不可能被 6整除 .  所以 只能基于 /3 再硬算
// /7 。。硬算吧
// /8 . 1000%8 = 0, 所以只需要看 最后3位。  或者说， for (i += 8) { 前x位，后x位 必须等于i，其他的位可以全排列 }    。。 不需要全排列， 反正 i+=8 会碰到的。 。。 出问题了， 6116 。
// /9 。 /3

// 直接构建， 不搞虚的。 力大砖飞！

// rust 的for 没办法 自定义步长啊。。只能while了。 或者 loop

// n 是偶数还是奇数，影响挺大的。

// rearrange...... 一下子崩了。。 要去重啊。 感觉还是 找 k-palindrome，然后 排序， 作为key 去重。



use std::collections::HashSet;

impl Solution {


    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        if n % 2 == 0 {
            return Self::cnt_good_for_even(n, k);
        } else {
            return Self::cnt_good_for_odd(n, k);
        }
    }

    fn cnt_good_for_even(n: i32, k: i32) -> i64 {
// let mut connected: HashSet<(i32, i32)> = HashSet::new();
// connected.insert((u, v));
// if connected.contains(&(i, j)) {

        let mut set2 : HashSet<i32> = HashSet::new();

        let mut sfx = 0i32;
        let mxsfx = 10i32.pow(n as u32/2);
        let mut ans = 0i64;
        while sfx + k < mxsfx {
            sfx += k;
            if sfx % 10 == 0 {
                continue;
            }
            // sfx % k == 0
            // 这里的 sfx 是不是 必然可以 作为回文的 后一半？ 还需要什么筛选吗？
            // 还需要： 4114 无法被 7 除尽 。 6116%8=4
            // 所以还是得 构建全部，无法通过 suffix 直接判断， 有些可以，比如 n=2。
            if Self::generate_check(sfx, n, k) {
                let key = Self::generate_key(sfx);
                if !set2.contains(&key) {
                    set2.insert(key);
                    ans += Self::count_rearrange(key);

                    println!("{}, {}", key, ans);

                }
            }
        }
        ans
    }

    fn count_rearrange(mut key: i32) -> i64 {
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        let mut arr = vec![0; 10];
        while key > 0 {
            if key % 10 == 0 {
                cnt0 += 2;  // +2
            } else {
                cnt1 += 2;
                arr[key as usize % 10] += 2;
            }
            key /= 10;
        }
        let mut ans = 1i64;

        while cnt0 > 0 {
            ans *= cnt1 as i64;
            cnt0 -= 1;
        }
        while cnt1 > 0 {
            ans *= cnt1 as i64;
            cnt1 -= 1;
        }
        for i in 1..10 {
            while arr[i] > 0 {
                ans /= arr[i] as i64;
                arr[i] -= 1;
            }
        }

        ans
    }

    fn generate_key(mut sfx: i32) -> i32 {
        let mut arr = vec![0; 10];
        while sfx > 0 {
            arr[sfx as usize % 10] += 1;
            sfx /= 10;
        }
        let mut ans = 0i32;
        for i in 0..10 {
            let t2 = 9 - i;
            while arr[t2] > 0 {
                arr[t2] -= 1;
                ans *= 10;
                ans += t2 as i32;
            }
        }
        ans
    }

    // 生成+判断， 返回 bool
    // n is even
    fn generate_check(mut sfx: i32, n: i32, k: i32) -> bool {
        let mut pw = 10i64.pow(n as u32);
        let mut ans = sfx as i64;

        while sfx > 0 {
            ans += pw * (sfx as i64 % 10);
            sfx = sfx / 10;
            pw = pw / 10;
        }

        ans % k as i64 == 0i64
    }

    fn cnt_good_for_odd(n: i32, k: i32) -> i64 {
        if n == 1 {
            return (9 / k + 1) as i64;
        }
        let mut sfx = 0i32;
        let mxsfx = 10i32.pow(n as u32/2);
        let mut ans = 0i64;
        while sfx + k < mxsfx {
            sfx -= 1;
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::count_good_integers(4,7));
}


