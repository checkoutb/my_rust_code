








// 上升， 下降
// 1 <= nums[i] <= 50

// ???

// 1-50  [0]- 1-50
// x-y  [1]- x-y

// 1-50  0 - 1-50
// 


// tle...  763/801

// + mni,  tle...  797/801   . all 49

// g

// hint:  dp[i][s],  arr1[i-1] = s
// 确实， vvi[t2][t3] 没有意义，在t2 确定的情况下， t3 只有一种可能，就是 [i-1] - t2

impl Solution {

    const MOD : i32 = 1000000007;
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let mut vvi = vec![vec![0; 51]; 51];
        for t2 in 0..=nums[0] as usize {
            vvi[t2][nums[0] as usize - t2] = 1;
        }
        let mut mni = 0usize;
        for idx in 1..nums.len() {
            let mut vv2 = vec![vec![0; 51]; 51];

            let mut mni2 = -1i32;

            for t2 in mni..=nums[idx] as usize { // [t2][nums - t2]
                let t3 = nums[idx] as usize - t2;

                let mut cnt = 0i32;
                for i in 0..=t2 {
                    for j in t3..=50 {
                        cnt = (cnt + vvi[i][j]) % Self::MOD;
                    }
                }

                vv2[t2][t3] = cnt;

                if mni2 == -1 && cnt != 0 {
                    mni2 = t2 as i32;
                }
            }
            if mni2 == -1 {
                return 0;
            }
            mni = mni2 as usize;
            vvi = vv2;
        }
        let mut ans = 0i32;
        for i in 0..51 {
            for j in 0..51 {
                ans = (ans + vvi[i][j]) % Self::MOD;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    // let vi = [2,3,2].to_vec();
    let vi = [5,5,5,5].to_vec();

    println!("ans: {:?}", Solution::count_of_pairs(vi));
}


