




// Runtime
// 11ms
// Beats100.00%
// Analyze Complexity
// Memory
// 3.91MB
// Beats100.00%

// cost() = [i] - [i+1] + [i+2] - [i+3]
// k can be any number !!!
// 任何时候， 我都可以 +， 只有前面是+的时候，我才可能-
impl Solution {
    pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
        if nums.len() == 1 {
            return nums[0] as i64;
        }
        let mut pos = (nums[0] + nums[1]) as i64; // last operation is add
        let mut neg = (nums[0] - nums[1]) as i64; // -

        for i in 2..nums.len() {
            let t2 = (pos + nums[i] as i64).max(neg + nums[i] as i64);
            let t3 = (pos - nums[i] as i64);
            pos = t2;
            neg = t3;
        }
        pos.max(neg)
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


