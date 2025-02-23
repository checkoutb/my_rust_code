

// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.22MB
// Beats36.84%

// 复制的时候， 分析复杂度也会复制进来，直接不会复制到的。。


// 好久没写rust了，好陌生。。

// 这题错了2次， while n > 0      n -= 1


impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut del: i32 = 10;
        let mut n = n;
        while n >= 0 {
            if n >= del {
                n -= del;
            } else {
                return false;
            }
            del -= 1;
            if n >= del {
                n -= del;
            } else {
                return true;
            }
            del -= 1;
        }
        unreachable!();
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


