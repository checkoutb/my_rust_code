





// Runtime14 ms
// Beats
// 60%
// Memory3 MB
// Beats
// 40%
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut en = 0;     // next will remove
        let mut st = nums.len() - 1;       // next will add
        let mut t2 = 0;
        while t2 < x && en < nums.len() {
            t2 +=nums[en];
            en += 1;
        }
        let mut ans = i32::MAX;

        if t2 == x {
            ans = en as i32;
            if en == nums.len() {       // st的原因会导致 st0,en0，就错了。
                return ans;
            }
        }
        // println!("111 {},{},{},{}", st, en, ans, t2);

        while en > 0 {
            en -= 1;
            t2 -= nums[en];
            //while st >= 0 && t2 + nums[st] <= x { // comparison is useless due to type limits
            // md 不写 st >= 0, arr[INT_MAX * 2 + 1] 了。
            // while st >= 0 && t2 + nums[st] <= x {
            while st >= en && t2 + nums[st] <= x {       // ...  >= en, not >= 0...
                t2 += nums[st];
                if st > 0 {
                    st -= 1;
                } else {
                    break;      // add [0], then add [0] again...
                }
            }
            if t2 == x {
                // println!("{},{},{}", st, en, ans);
                ans = ans.min((nums.len() - st + en - 1) as i32);
            }
            // if st > nums.len() {        // 
            //     break;
            // }
        }

        if ans == i32::MAX {
            return -1;
        } else {
            return ans;
        }
    }
}




struct Solution {}

fn main()
{
    // let vi = [5,1,4,2,3].to_vec();
    // let x = 2;

    // let vi = [3,2,20,1,1,3].to_vec();
    // let x = 10;

    // let vi = [1,1].to_vec();
    // let x = 3;

    // 16
    let vi = [8828,9581,49,9818,9974,9869,9991,10000,10000,10000,9999,9993,9904,8819,1231,6309].to_vec();
    let x = 134365;

    println!("ans: {:?}", Solution::min_operations(vi, x));
}


