




// D D

// 从头开始计算就可以了，不需要 预先xor 所有元素。


// Runtime49 ms
// Beats
// 18.18%
// Memory3.7 MB
// Beats
// 18.18%
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor: i32 = 0;
        for i in 0..nums.len() {
            xor ^= nums[i];
        }
        let mx: i32 = (1 << (maximum_bit)) - 1;
        let mut ans: Vec<i32> = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            ans[i] = mx ^ xor;
            xor ^= nums[i];
        }
        ans.reverse();
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [0,1,1,3].to_vec();
    let mxbit = 2;

    println!("ans: {:?}", Solution::get_maximum_xor(vi, mxbit));
}


