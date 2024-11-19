






// Runtime
// 38ms
// Beats85.11%
// Analyze Complexity
// Memory
// 4.52MB
// Beats31.91%

impl Solution {
    pub fn maximum_total_sum(mut maximum_height: Vec<i32>) -> i64 {
        maximum_height.sort_unstable();
        maximum_height.reverse();

        for i in 1..maximum_height.len() {
            maximum_height[i] = maximum_height[i].min(maximum_height[i - 1] - 1);
        }

        if maximum_height[maximum_height.len() - 1] <= 0 {
            return -1i64;
        }
        maximum_height.iter().map(|i| *i as i64).sum::<i64>()
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


