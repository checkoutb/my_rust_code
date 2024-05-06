

// D 

// gcd 是错的，确实，下面这个 应该是 4,但是我返回8... 网上说 gcd会返回 2.
// aabbabab

// for key, value in hashmap.items():
//     div = gcd(div, value)
// return n // div
// 。。。这么用的。。


// mxsublen 的问题，搞混了。这个是作为上界，最开始的时候是 s.len() / 2 
//  后来就发现， sublen 的长度 必须能 整除 所有的 vi中元素 和 s.len， 所以 搞了一个 gcd
//  当时 直接用 gcd作为 mxsublen, 但是 pqqppqpqpq 这个失败了。 所以改成 s.len / gcd 。。。
//  搞不清，mxsublen 的 gcd 有没有关系，有什么关系了。 所以 gcd 只是上界，不能用于 % sublen ？
// 不行， gcd 不能作为上界 lbuorourlb 
// 上界 使用 s.len() / 2 是可以的。
// 所以这题目，力大砖飞吗？

// 还真是， hint1: 答案肯定可以整除s.len， hint2: 尝试所有的

// 32%的通过率 吓到我了。 不敢穷举。。。


// Runtime
// 54ms
// Beats100.00%of users with Rust
// Memory
// 15.28MB
// Beats100.00%of users with Rust
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        
        use std::collections::HashMap;
        
        let mut map2 = HashMap::new();
        for ch in s.chars() {
            map2.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1i32);
        }
        let mut vi = map2.into_values().collect::<Vec<_>>();
        vi.sort();

        if vi.len() == 1 {
            return 1i32;
        }

        let mut sublen = vi.len();  // hai keyi jingque
        // let mxsublen = s.len() / vi[0];
        // let mxsublen = s.len() / Self::gcd2(&vi, s.len() as i32) as usize;
        let mxsublen = Self::gcd2(&vi, s.len() as i32) as usize;
        let mut sit = s.chars();
        let mut vvi = vec![vec![0;26]; s.len()];
        vvi[0][sit.next().unwrap() as usize - 97usize] = 1;

        for i in 1..vvi.len() {
            for j in 0..26 {
                vvi[i][j] = vvi[i - 1][j];
            }
            vvi[i][sit.next().unwrap() as usize - 97usize] += 1;
        }
        // println!("{}, {}, {:?}", sublen, mxsublen, vi);
        // while sublen <= mxsublen {
        while sublen <= s.len() / 2 {
            // println!("{}, {}", sublen, mxsublen);
            // if mxsublen % sublen != 0 {
            if s.len() % sublen != 0 {
                sublen += 1;
                continue;
            }
            
            let mut idx = sublen + sublen;
            let mut can = true;
            while idx <= s.len() {
                for i in 0..26 {

                    // vi. vvi. ...
                    if vvi[sublen - 1][i as usize] != (vvi[idx - 1][i as usize] - vvi[idx as usize - sublen as usize - 1usize][i as usize])
                    {
                        can = false;
                        break;
                    }
                }
                idx += sublen;
            }

            if can {
                return sublen as i32;
            }
            
            sublen += 1;
            // println!("{}, {}", sublen, mxsublen);
        }

        s.len() as i32
    }

    fn gcd2(vi: &Vec<i32>, mut t2: i32) -> i32 {
        
        for i in vi {
            t2 = Self::gcd3(*i, t2);
        }
        t2
    }

    fn gcd3(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Self::gcd3(b, a % b);
    }
}

struct Solution {}

fn main()
{

    // let s = "pqqppqpqpq".to_string();       // 2
    // let s = "aabbabab".to_string();  // 4
    let s = "lbuorourlb".to_string();

    println!("ans: {:?}", Solution::min_anagram_length(s));
}


