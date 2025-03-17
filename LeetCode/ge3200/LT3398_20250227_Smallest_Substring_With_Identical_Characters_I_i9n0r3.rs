


// D

// 只需要考虑特殊情况： 长度为1.  其他的不需要考虑，包括 修改自己 还是 修改前面哪个
// 不， 。。 完全不同， D中是 最大长度/(k+1)。 并不是 碰到 k+1个连续，就 尝试 修改自己 或修改前面，  而是 知道 最大 相同值的长度后 ，才计算 需要多少次 。。。。。。。




// 0011100

// 00000



// g


// 二分？
// 000 11  md=2   00111  00110  . ....   01011
// 001110    

// 1001  1

// 00110 2  01010

impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        
        let s = s.into_bytes();
        let au0 = b'0';
        let au1 = b'1';

        let mut st = 1;
        let mut en = s.len();
        let mut ans = i32::MAX;
        while st <= en {
            let md = (st + en) / 2;

            println!("{} {} {}", st, en, md);

            if Self::cana1(&s, 0, b'2', 0, md as i32, 0, num_ops) <= num_ops {
                ans = ans.min(md as i32);
                en = md - 1;
            } else {
                st = md + 1;
            }

            // let mut same_cnt = 1;
            // let mut op_cnt = 0;
            // let mut lst = s[0];
            // for i in 1..s.len() {
            //     if s[i] == lst {
            //         same_cnt += 1;
            //         if same_cnt > md {
            //             if (md != 1 || i == 1) && i + 1 < s.len() && s[i + 1] != s[i] {
            //                 lst = s[i]; // change s[i-1]
            //             } else {
            //                 if s[i] == au0 {
            //                     lst = au1;
            //                 } else {
            //                     lst = au0;
            //                 }
            //             }
            //             op_cnt += 1;
            //             same_cnt = 1;
            //             if op_cnt > num_ops {
            //                 break;
            //             }
            //         }
            //     } else {
            //         same_cnt = 1;
            //         lst = s[i];
            //     }
            // }
            // if op_cnt <= num_ops {
            //     ans = ans.min(md as i32);
            //     en = md - 1;
            // } else {
            //     st = md + 1;
            // }
        }
        ans
    }

    fn cana1(s: &Vec<u8>, idx: usize, lst: u8, sz: i32, mx: i32, op: i32, opmx: i32) -> i32 {
        println!("{} {} {} {}", idx, lst, sz, op);
        if idx == s.len() {
            return op;
        }
        if op > opmx {
            return op;
        }
        // let mut ans0 = i32::MAX;
        // let mut ans1 = i32::MAX;
        let mut ans = i32::MAX;
        if s[idx] == lst {
            if sz + 1 > mx {
                ans = ans.min(Self::cana1(s, idx + 1, if s[idx] == b'0' { b'1' } else { b'0' }, 1, mx, op + 1, opmx));
                if mx > 1 {
                    ans = ans.min(Self::cana1(s, idx + 1, s[idx], 1, mx, op + 1, opmx));
                } else if idx < 2 {
                    ans = ans.min(Self::cana1(s, idx + 1, s[idx], 1, mx, op + idx as i32, opmx));
                }
            } else {
                ans = ans.min(Self::cana1(s, idx + 1, s[idx], sz + 1, mx, op, opmx));
            }
        } else {
            ans = ans.min(Self::cana1(s, idx + 1, s[idx], 1, mx, op, opmx));
        }
        ans
    }
}


struct Solution {}

fn main()
{

    // let s = "00110".to_string();
    // let n = 2;

    let s = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
    let n = 36;

    println!("ans: {:?}", Solution::min_length(s, n));
}


