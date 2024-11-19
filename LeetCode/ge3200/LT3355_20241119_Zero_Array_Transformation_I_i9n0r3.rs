



// Runtime
// 14ms
// Beats60.61%
// Analyze Complexity
// Memory
// 9.97MB
// Beats42.42%


// 选择 [l, r] 中的一部分下标，然后这些值 -1
// 执行完全部，   。。。 这一下子似乎很简单了啊。  估计 II 是问你 最早 哪个query之后 就全0 ？.. 真是。

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut ops = vec![0; nums.len() + 1];

        for i in 0..queries.len() {
            ops[queries[i][0] as usize] -= 1;
            ops[queries[i][1] as usize + 1] += 1;
        }

        let mut diff = 0;
        for i in 0..nums.len() {
            diff += ops[i];
            if nums[i] + diff > 0 {
                return false;
            }
        }

        true
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


