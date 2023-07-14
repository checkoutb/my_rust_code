



// D D

// let mut bits = [0; 32];
// for n in nums.iter() {
//   for i in 0..32 {
//     bits[i] += (n >> i) & 1;
//     bits[i] %= 3;
//   }
// }
// bits.iter().rev().fold(0, |b, x| b*2 + x)



// let mut ones = 0;
// let mut twos = 0;
// for n in nums {
//     ones = (ones ^ n) & !twos;
//     twos = (twos ^ n) & !ones;
// }
// return ones;



// Runtime
// 4 ms
// Beats
// 8.33%
// Memory
// 2.2 MB
// Beats
// 95.83%

// bit % 3
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut vi: Vec<i32> = vec![0; 32];     // 1 << 31
        for n in nums.iter() {
            for bt in 0..32 {
                if n & (1 << bt) != 0 {
                    vi[bt] += 1;
                }
            }
        }
        let mut ans: i32 = 0;
        for bt in 0..32 {
            if vi[bt] % 3 == 1 {
                ans |= 1 << bt;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    let vi = [2,2,2,3,-123,3,3].to_vec();

    println!("ans: {:?}", Solution::single_number(vi));
}


