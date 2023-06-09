
// D D


// use std::cmp;
// let mut arr = vec![a, b, c];
// arr.sort();
// return cmp::min((arr[0]+arr[1]+arr[2])/2, arr[0]+arr[1]);




// Runtime1 ms
// Beats
// 75%
// Memory2.3 MB
// Beats
// 25%
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let sum2 = a + b + c;
        let mx = a.max(b.max(c));
        if mx > sum2 - mx {
            sum2 - mx
        } else {
            sum2 / 2
        }
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


