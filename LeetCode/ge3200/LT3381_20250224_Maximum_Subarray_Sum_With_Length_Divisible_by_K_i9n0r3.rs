


// let mut window_sum = nums.iter().take(k).sum::<i64>();



// Runtime
// 9ms
// Beats33.33%
// Memory
// 6.75MB
// Beats33.33%




impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut vi = vec![];
        let mut cnt = 0;
        let mut sum2 = 0i64;
        for i in 0..nums.len() {
            sum2 += nums[i] as i64;
            cnt += 1;
            if cnt > k {
                sum2 -= nums[i - k as usize] as i64;
                cnt -= 1;
            }
            if cnt == k {
                vi.push(sum2);
            }
        }

        // println!("{:?}", vi);

        let mut ans = i64::MIN;
        for i in 0..(k) as usize {
            sum2 = 0;
            let mut idx = i;
            while idx < vi.len() {
                sum2 += vi[idx];
                ans = ans.max(sum2); // 在 if sum2 < 0 上面!!
                if sum2 < 0 {
                    sum2 = 0;
                }
                idx += k as usize;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [1,2].to_vec();
    let k = 1;

    println!("ans: {:?}", Solution::max_subarray_sum(vi, k));
}


