






// g

// while st + 50 <= en {   出错
//          + 100          tle


// 无法理解 怎么3步搞定 gigigjjggjjgg， 这个是 7，0，2，4
// 除非 题目里说的 its next letter 是指 后续的全部都可以一次转换。
// 神坑。  .. 也不是 后续的全可以，以为 aaabbc 是返回2. 并不是直接 a->c
// 那怎么搞定 3步搞定 7024  。。靠，知道了 7006  7007 。 ++


// count always vi[i] ? or  0 - vi.max()  ..... 0-vi.max....  得二分，不然肯定tle的。
// 唯一的问题是 md 和 md+1 必然不同吗？ 先当不同算。
// 不过  0-vi.max 应该也可以吧？  no. tle...
// md == md+1 .....


impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let s = s.as_bytes();

        let au8 = b'a';
        let mut vi = vec![0; 26];
        let mut st = 0;
        let mut en = 0;
        for i in 0..s.len() {
            vi[(s[i] - au8) as usize] += 1;
            en = en.max(vi[(s[i] - au8) as usize]);
        }

        // println!("{:?}", vi);

        let mut ans = i32::MAX;
        let mut stans = Self::dfsa1(&vi, 0, 0, st);
        let mut enans = Self::dfsa1(&vi, 0, 0, en);
        while st + 50 <= en {
            println!("st {} en {}", st, en);
            let md = (en - st) * 1 / 4;
            let md2 = (en - st) * 3 / 4;

            let t2 = Self::dfsa1(&vi, 0, 0, md);
            let t3 = Self::dfsa1(&vi, 0, 0, md2);
            let tmd = Self::dfsa1(&vi, 0, 0, (md + md2) / 2);

            println!(" md {} {} {}", md, md2, (md + md2) / 2);

            ans = ans.min(t2.min(t3.min(tmd)));
            if t2 < t3 {
                if tmd > t2 {
                    en = (md + md2) / 2 - 1;
                } else if tmd == t2 {
                    en = md2 - 1;
                } else {
                    st = t2 + 1;
                    en = t3 - 1;
                }
            } else if (t2 == t3) {
                if tmd < t2 {
                    st = t2 + 1;
                    en = t3 - 1;
                } else {
                    break;  // t2==t3==tmd
                }
            } else {
                if tmd > t3 {
                    st = (md + md2) / 2 + 1;
                } else if tmd == t3 {
                    st = md + 1;
                } else {
                    st = md + 1;
                    en = md2 - 1;
                }
            }

            // let md = (st + en) / 2;
            // let t2 = Self::dfsa1(&vi, 0, 0, md);
            // let t3 = Self::dfsa1(&vi, 0, 0, md + 1);
            // if (t2 < t3) {
            //     en = t2 - 1;
            //     ans = ans.min(t2);
            // } else {
            //     st = md + 2;
            //     ans = ans.min(t3);
            // }

            // if (t2 == t3) {
            //     println!("===================");
            // }
        }

        while st <= en {
            ans = ans.min(Self::dfsa1(&vi, 0, 0, st));
            st += 1;
        }



        // tle
        // while st <= en {
        //     ans = ans.min(Self::dfsa1(&vi, 0, 0, st));
        //     st += 1;
        // }


        // for i in 0..vi.len() {
        // for i in 0..10 {
        //     // if vi[i] == 0 {
        //     //     continue;
        //     // }
        //     // let tar = vi[i];
        //     let tar = i as i32;
        //     let t2;
        //     // let mut remain = 0;
        //     // for j in 0..vi.len() {
        //     //     if vi[j] == 0 {
        //     //         t2 += remain;
        //     //         remain = 0;
        //     //         continue;
        //     //     }
        //     //     if vi[j] > tar {
        //     //         t2 += remain;
        //     //         remain = vi[j] - tar;
        //     //     } else if vi[j] < tar {
        //     //         if remain >= tar - vi[j] {
        //     //             // t2 += tar - vi[j];
        //     //             t2 += remain;   // some change to next letter, other delete
        //     //         } else {
        //     //             t2 += tar - vi[j];
        //     //         }
        //     //         remain = 0;
        //     //     } else {
        //     //         t2 += remain;
        //     //         remain = 0;
        //     //     }
        //     // }
        //     // t2 += remain;

        //     t2 = Self::dfsa1(&vi, 0, 0, tar);

        //     println!("{} {}", t2, tar);

        //     ans = ans.min(t2);
        // }

        ans

    }

    fn dfsa1(vi: &Vec<i32>, mut remain: i32, idx: usize, tar: i32) -> i32 {
        // println!(">>> {}, {}", remain, idx);
        if idx == vi.len() {
            return remain;
        }
        let mut ans = 0;
        if vi[idx] == 0 {
            // println!(" 000000 {}", idx);
            ans += remain;
            ans += Self::dfsa1(vi, 0, idx + 1, tar);
        } else if vi[idx] >= tar {
            // println!("     111111 {}", idx);
            ans += remain;
            ans += Self::dfsa1(vi, vi[idx] - tar, idx + 1, tar);
        } else {
            // println!("         222222 {}", idx);
            // delete or insert

            // delete
            let mut t3 = 0;
            t3 += remain;
            // t3 += vi[idx];
            t3 += Self::dfsa1(vi, vi[idx], idx + 1, tar);

            // insert / upper
            let mut t2 = 0;
            if remain > tar - vi[idx] {
                t2 += remain;
                t2 += Self::dfsa1(vi, 0, idx + 1, tar);
            } else {
                t2 += tar - vi[idx];
                t2 += Self::dfsa1(vi, 0, idx + 1, tar);
            }
            ans = t2.min(t3);
        }
        // println!("    {}, {}", idx, ans);
        ans
    }
}


struct Solution {}

fn main()
{

    // let s = "gigigjjggjjgg".to_string();
    // let s = "acabyyyyyyyzzz".to_string();
    let s = "qdddddddqddddddqddbqdbdqqqbdqbqdqdbqdddqbqbddddddddbbddqdqddddqdqqbddqqqdbdqdqddddqdddqbdqqdqqdqdb".to_string();

    println!("ans: {:?}", Solution::make_string_good(s));
}


