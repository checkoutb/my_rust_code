











// Runtime
// 5ms
// Beats100.00%
// Memory
// 2.25MB
// Beats100.00%



impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {


        let mut ans = -1;
        for i in 0..nums.len() {
            let mut vb = vec![false; 1001];
            let t2 = nums[i];
            if t2 == 0 {    // .....
                ans = ans.max(0);
                continue;
            }

            vb[0] = true;
            for j in 0..queries.len() {

                if queries[j][0] as usize > i || (queries[j][1] as usize) < i {
                    continue;
                }
                let t3 = queries[j][2];
                if t3 > t2 {
                    continue;
                }
                for k in (0..=(t2 - t3) as usize).rev() {
                    if vb[k] {
                        vb[k + t3 as usize] = true;
                    }
                }
                
                if vb[t2 as usize] {
                    ans = ans.max(j as i32 + 1);
                    break;
                }
            }
            if !vb[t2 as usize] {
                return -1;
            }
        }

        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


