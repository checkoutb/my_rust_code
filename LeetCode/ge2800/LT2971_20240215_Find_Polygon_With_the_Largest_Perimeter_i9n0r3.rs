
// D D

// for i in nums {
//     if i64::from(i) < tt - i as i64 {
//         return tt as i64;
//     }
//     tt -= i64::from(i);
// }


// Runtime
// 32ms
// Beats23.08%of users with Rust
// Memory
// 4.08MB
// Beats94.87%of users with Rust

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut ans = -1;
        let mut nums = nums;
        nums.sort();
        let mut sum2 = (nums[0] + nums[1]) as i64;
        for i in 2..nums.len() {
            let t2 = nums[i] as i64;
            if sum2 > t2 {
                ans = sum2 + t2;
            }
            sum2 += t2;
        }
        ans
    }
}




struct Solution {}

fn main()
{
    let vi = [5,5,5].to_vec();

    println!("ans: {:?}", Solution::largest_perimeter(vi));
}


