





// Runtime1 ms
// Beats
// 83.50%
// Memory2.2 MB
// Beats
// 6.80%
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        let mut t2: i32 = 0;
        for i in gain {
            t2 += i;
            ans = ans.max(t2);
        }
        ans
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


