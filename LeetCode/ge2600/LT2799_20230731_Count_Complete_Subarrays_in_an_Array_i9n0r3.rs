





// 

// Runtime4 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.1 MB
// Beats
// 100%

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut vi = vec![0; 2001];
        let sz1 = nums.len();
        let mut st = 0;
        let mut cnt = 0;
        for i in 0..sz1 {
            if vi[nums[i] as usize] == 0 {
                cnt += 1;
                vi[nums[i] as usize] += 1;
            }
        }
        let mut ans = 0;
        for i in 0..sz1 {
            if vi[nums[i] as usize] == 1 {
                cnt -= 1;
            }
            vi[nums[i] as usize] += 1;
            while cnt == 0 {
                ans += sz1 - i;
                vi[nums[st] as usize] -= 1;
                if vi[nums[st] as usize] == 1 {
                    cnt += 1;
                }
                st += 1;
            }
        }
        return ans as i32;
    }
}


struct Solution {}

fn main()
{
    // let vi = [1,3,1,2,2].to_vec();
    let vi = [5,5,5,5].to_vec();

    println!("ans: {:?}", Solution::count_complete_subarrays(vi));
}


