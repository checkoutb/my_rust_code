


// D D

// Arrays.sort(integerNums, (a, b) -> {
//     String binA = Integer.toBinaryString(a);
//     String binB = Integer.toBinaryString(b);
//     return (binB + binA).compareTo(binA + binB);
// });




// Runtime
// 2ms
// Beats63.16%
// Analyze Complexity
// Memory
// 2.12MB
// Beats32.20%

impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for i in 0..3 {
            for j in 0..3{
                if i == j { continue; }
                for k in 0..3 {
                    if i == k || j == k { continue; }
                    ans = ans.max(Self::concate(nums[i], nums[j], nums[k]));
                }
            }
        }
        ans
    }

    fn concate(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;
        let mut idx = 0;
        while a > 0 {
            ans |= (a & 1) << idx;
            a >>= 1;
            idx += 1;
        }
        while b > 0 {
            ans |= (b & 1) << idx;
            b >>= 1;
            idx += 1;
        }
        while c > 0 {
            ans |= (c & 1) << idx;
            c >>= 1;
            idx += 1;
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


