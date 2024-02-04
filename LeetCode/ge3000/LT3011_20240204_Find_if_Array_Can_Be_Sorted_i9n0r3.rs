

// D D

// let ones = n.count_ones();



// Runtime
// 4ms
// Beats43.48%of users with Rust
// Memory
// 2.21MB
// Beats76.09%of users with Rust
impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut vi = vec![0; nums.len()];

        for i in 0..nums.len() {
            vi[i] = Self::bit_cnt(nums[i]);
        }

        // let mut mn = 1024;
        let mut mx = -1;
        let mut mn2 = nums[0];
        let mut mx2 = nums[0];

        // println!(" {:?}, \n {:?}", nums, vi);

        for i in 1..nums.len() {
            if vi[i] != vi[i - 1] {
                if mn2 < mx {
                    return false;
                }
                mx = mx2;
                // mn = mn2;
                mn2 = nums[i];
                mx2 = nums[i];
            } else {
                mn2 = mn2.min(nums[i]);
                mx2 = mx2.max(nums[i]);
            }
        }

        mn2 >= mx
    }

    fn bit_cnt(mut a: i32) -> i32 {             // let ones = n.count_ones();
        let mut cnt = 0;
        while a > 0 {
            cnt += a % 2;
            a  = a >> 1;
        }
        cnt
    }
}


struct Solution {}

fn main()
{

    // let vi = [1,2,3,4,5].to_vec();
    // let vi = [3,16,8,4,2].to_vec();
    let vi = [8,4,2,30,15].to_vec();

    println!("ans: {:?}", Solution::can_sort_array(vi));
}


