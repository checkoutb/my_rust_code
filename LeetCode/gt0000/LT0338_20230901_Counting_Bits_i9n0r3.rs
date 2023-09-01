


// let mut power = 0;
// let mut curr_range:i32 = 0;
// for i in 1..=n {
//     if i == 2i32.pow(power) {
//         result.push(1);
//         curr_range = 2i32.pow(power);
//         power += 1;
//     } else {
//         result.push(result[(i - curr_range) as usize] + 1)
//     }   
// }
// 6


// Runtime5 ms
// Beats
// 54.42%
// Memory2.6 MB
// Beats
// 76.19%

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut vi = vec![0; n as usize + 1];
        for i in 1..=(n as usize) {
            vi[i] = Self::get_cnt(i as i32);
        }
        vi
    }

    fn get_cnt(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            n = n & (n - 1);
            ans += 1;
        }
        ans
    }
}



struct Solution {}

fn main()
{
    let n = 5;

    println!("ans: {:?}", Solution::count_bits(n));
}


