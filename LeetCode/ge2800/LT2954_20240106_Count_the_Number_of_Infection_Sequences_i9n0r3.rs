

// D

// 多个组，每个组内的相对顺序不变，一共有：  (g1 + g2 + g3... gn)! / (g1! * g2! ... * gn!)
// gx 代表 第x个组的 元素个数
// 每个组内的顺序 就是 2^(t2-1)



// g, 不能用 nCr

// tle ...

// hint： 
// we have two choices per second since the children at the two endpoints can both be the infect candidates. 
// So there are 2 ^ (j - i + 1) orders to infect all children in the segment.

// tle... 无解， 感觉还是 t4 的计算 有问题。

// t4 的 计算逻辑：
// 主要是 不关心 被谁感染的，所以 会 出现了 重复。
// 以 5, [0,4] 为例：  1被0感染， 23被4感染， 那么可能出现 132 的顺序
//                    如果12被0感染，3被4感染， 也会出现  132 的顺序， 出现了重复。
// 后来想到： 最后一个被感染的人 可能被前面，也可能被后面 的人 感染，所以 就剔除这人 (就是这人固定在末尾)。 使用 t2-1 C j 来算， 而不是之前的 t2 C j ，(t2 C j 等于 t2 C t3)
// 就是从原来的 for j in 0..=t2 ， 前 j 个人 被 前面的人感染， 后面t3个人 被后面的人感染。
//      变成 for j in 0..t2 , [j] 这个人是 最后一个 被感染的人， 但不关心 他被谁感染。  [j]之前的肯定是被前面的人感染，[j]之后的 肯定被后面的人感染。


// 每次感染一个，求所有可能的感染顺序。
// 需要计算大数的排列。 需要2次。
impl Solution {
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007i64;
        let mut ans = 1i64;
        // let pfx = sick[0];      // site
        let mut cnt = sick[0];      // child not sick
        let mut t2;
        // let mut t3;
        let mut t4;
        for i in 1..sick.len() {
            t2 = sick[i] - sick[i - 1] - 1;

            if t2 == 0 {
                continue;       // t2==0 -> t4==0 -> ans==0...
            }

            t4 = 0i64;

            t4 = Self::p2owNModp(t2 - 1, MOD);

            // for j in 0..t2 {
            //     t4 = (t4 + Self::nCrModp(t2 as i64 - 1i64, j as i64, MOD)) % MOD;
            // }


            // wo ++
            //
            // for j in 0..=t2 {
            //     t3 = t2 - j;        // j: sicked from [i-1],  t3: socked from [i]

            //     // if j == t3 {
            //     //     t4 = (t4 + Self::nCrModp(t2 as i64, t3 as i64, MOD) * 2) % MOD;
            //     // } else {
            //         t4 = (t4 + Self::nCrModp(t2 as i64, t3 as i64, MOD)) % MOD;
            //     // }
                
            //     println!("{},{},{}", t2, t3, t4);
            // }
            // t4 /= 2;
            ans = (ans * Self::nCrModp(cnt as i64 + t2 as i64, t2 as i64, MOD)) % MOD;
            
            // println!("{},{}", ans, t4);

            ans = ans * t4 % MOD;
            cnt += t2;
        }
        if *sick.last().unwrap() != n - 1 {
            t2 = n - 1 - sick.last().unwrap();
            ans = (ans * Self::nCrModp(cnt as i64 + t2 as i64, t2 as i64, MOD)) % MOD;

            // println!("{} {} {}   {}", cnt, t2, ans, Self::nCrModp(cnt as i64 + t2 as i64, t2 as i64, MOD));
            // println!("{}", Self::nCrModp(3i64, 1i64, MOD));
        }
        ans as i32
    }

    fn p2owNModp(mut n: i32, p: i64) -> i64 {
        let mut ans = 1i64;
        while n > 0 {
            ans = (ans << 1) % p;
            n -= 1;
        }
        ans
    }

    fn nCrModp(n: i64, mut r: i64, p: i64) -> i64 {
        if r > n - r {
            r = n - r
        }
        if r == 0 {
            return 1;
        }
        // if (n / p - (n - r) / p > r / p) {
        //     return 0;
        // }
        let mut result = 1i64;
        let mut i = n;
        let mut x = 1;
        while i > r {
            if i % p != 0 {
                result = (result * (i % p)) % p;
            }
            if x % p != 0 {
                result = result * Self::inverseModp(x % p, p) % p;
            }
            i -= 1;
            x += 1;
        }
        result
    }

    fn inverseModp(mut a: i64, p: i64) -> i64 {
        let mut ex = p - 2;
        let mut result = 1;
        while ex > 0 {
            if ex % 2 == 1 {
                result = (result * a) % p;
            }
            a = (a * a) % p;
            ex /= 2;
        }
        result
    }
}



struct Solution {}

fn main()
{

    // let n = 5;
    // let v = [0,4].to_vec();     // 4

    // let n = 4;
    // let v = [1].to_vec();        // 3

    let n = 502;
    let v = [0,4,111,222,433,499].to_vec();     // 767568886

    // let n = 5;
    // let v = [0,1].to_vec();     // 1

    println!("ans: {:?}", Solution::number_of_sequence(n, v));
}


