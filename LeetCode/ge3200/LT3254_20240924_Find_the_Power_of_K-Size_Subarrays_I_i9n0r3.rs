




// II
//
// Runtime
// 53ms
// Beats48.39%
// Analyze Complexity
// Memory
// 3.51MB
// Beats79.03%




// ===================


// D D

// if (i && nums[i - 1] + 1 == nums[i])
//      score++;
// else
//      score = 0;
// if (score >= k - 1)
//      ans[i - k + 1] = nums[i];


// Runtime
// 5ms
// Beats26.23%
// Analyze Complexity
// Memory
// 2.17MB
// Beats73.77%


// k window, 3,4,5,6,7

// last not consecutive or sort
// the begin of consecutive and sort

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }
        let mut st = 0usize;
        let mut vi = vec![-1; nums.len() - k as usize + 1];
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                st = i;
                continue;
            }
            if i - st + 1 >= k as usize {
                vi[i - k as usize + 1] = nums[i];
            }
        }
        vi
    }
}

struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


