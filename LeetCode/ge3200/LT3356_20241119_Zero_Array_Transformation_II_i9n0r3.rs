





// Runtime
// 42ms
// Beats25.00%
// Analyze Complexity
// Memory
// 9.72MB
// Beats100.00%


// 二分？  nlogn 应该可以


impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {

        let mut ans = -1i32;
        let mut st = 0usize;
        let mut en = queries.len();

        while st <= en {
            let md = (st + en) / 2;

            if Self::can_zero(&nums, &queries, md) {
                ans = md as i32;
                if md == 0 {
                    break;
                }
                en = md - 1;
            } else {
                st = md + 1;
            }
        }

        ans
    }

    // < md
    fn can_zero(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, md: usize) -> bool {

        let mut ops = vec![0; nums.len() + 1];

        for i in 0..md {
            ops[queries[i][0] as usize] -= queries[i][2];
            ops[queries[i][1] as usize + 1] += queries[i][2];
        }

        let mut op = 0;
        for i in 0..nums.len() {
            op += ops[i];
            if nums[i] + op > 0 {
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


