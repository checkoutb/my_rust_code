

// D D

// let l = nums.partition_point(|&x| x < target);
// let r = nums.partition_point(|&x| x <= target);




// Runtime0 ms
// Beats
// 100%
// Memory2.4 MB
// Beats
// 65.83%
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1; 2];
        }
        let mut ans = vec![-1; 2];
        let t2 = Self::bsa1(&nums, target);
        if t2 == -1 {
            return ans;
        }

        let t3 = Self::bsa2(&nums, target);
        ans[0] = t2;
        ans[1] = t3;
        ans
    }

    fn bsa2(nums: &Vec<i32>, tar: i32) -> i32 {
        let mut st = 0 as i32;
        let mut en = nums.len() as i32 - 1;     // en can be -1.
        let mut ans = 0 as i32;
        while st <= en {
            let md = (st + en) / 2;
            if nums[md as usize] <= tar {
                ans = md;
                st = md + 1;
            } else {
                en = md - 1;
            }
        }
        if nums[ans as usize] == tar {
            return ans;
        }
        return -1;
    }

    fn bsa1(nums: &Vec<i32>, tar: i32) -> i32 {
        let mut st = 0 as i32;
        let mut en = nums.len() as i32 - 1;     // en can be -1.
        let mut ans = 0 as i32;
        while st <= en {
            let md = (st + en) / 2;
            if nums[md as usize] < tar {
                st = md + 1;
            } else {
                ans = md;
                en = md - 1;
            }
        }
        if nums[ans as usize] == tar {
            return ans;
        }
        return -1;
    }
}




struct Solution {}

fn main()
{
    let vi = [5,7,7,8,8,10].to_vec();
    let tar = 8;


    println!("ans: {:?}", Solution::search_range(vi, tar));
}


