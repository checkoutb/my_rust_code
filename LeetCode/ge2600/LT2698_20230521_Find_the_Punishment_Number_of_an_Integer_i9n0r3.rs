





// Runtime20 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory1.9 MB
// Beats
// 100%

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans: i32 = 0;

        for i in 1..=n {
            if Self::dfsa1(i, i * i) {
                ans += i * i;
            }
        }

        ans
    }

    fn dfsa1(rem: i32, ii: i32) -> bool {
        if rem < 0 {
            return false;
        }
        if rem == 0 {
            if ii == 0 {
                return true;
            } else {
                return false;
            }
        }
        if ii == 0 {
            return false;
        }
        let mut t2 = 1;
        while ii >= t2 {
            t2 *= 10;
            if Self::dfsa1(rem - ii % (t2), ii / t2) {
                return true;
            }
        }

        return false;
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::punishment_number(10));
}


