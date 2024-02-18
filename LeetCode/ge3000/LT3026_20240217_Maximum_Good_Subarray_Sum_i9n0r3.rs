





// Runtime
// 43ms
// Beats100.00%of users with Rust
// Memory
// 4.91MB
// Beats48.08%of users with Rust

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut psum = vec![0_i64; nums.len()];
        psum[0] = nums[0] as i64;
        // let mut mapmx : std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        let mut mapmn : std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        let mut ans = i64::MIN;
        mapmn.insert(nums[0], nums[0] as i64);
        for i in 1..nums.len() {
            psum[i] = psum[i - 1] + nums[i] as i64;
            // count.entry(e).and_modify(|c| *c += 1).or_insert(1);
            // mapmx.entry(nums[i]).and_modify(|v| *v = (*v).max(psum[i])).or_insert(psum[i]);
            mapmn.entry(nums[i]).and_modify(|v| *v = (*v).min(psum[i])).or_insert(psum[i]);
            let mx1 = psum[i];
            // let mx1 = mapmx.get(&nums[i]).unwrap();      // max sum, now_prefix_sum - mn.      mapmx's index maybe < mapmn's index,  just now prefix sum
            // let mn1 = mapmn
            // if mapmn.contains_key(&(nums[i] + k)) {
            //     let mn2 = mapmn.get(&(nums[i] + k)).unwrap();
            // }
            // if mapmn.contains_key(&(nums[i] - k)) {

            // }

            if let Some(&mn) = mapmn.get(&(nums[i] + k)) {
                ans = ans.max(mx1 - mn + (nums[i] + k) as i64);
            }
            if let Some(&mn) = mapmn.get(&(nums[i] - k)) {
                ans = ans.max(mx1 - mn + (nums[i] - k) as i64);
            }
        }
        // println!("{:?} \n\n {:?}", 111, mapmn);

        // // for (key, mx1) in mapmx.clone().into_iter() {
        // for (key, mx1) in mapmx.iter() {
        //     let mn1 = mapmn.get(&key).unwrap();
        //     if mapmn.contains_key(&(key + k)) {
        //         let mx2 = mapmx.get(&(key + k)).unwrap();
        //         let mn2 = mapmn.get(&(key + k)).unwrap();
        //         ans = ans.max(mx1 - mn2).max(mx2 - mn1);     // mn1/mx1/mn2/mx2  is <=[i], so need - [i], but what is i ? mn1's index or mx2's index ?
        //     }
        //     if mapmn.contains_key(&(key - k)) {
        //         let mx2 = mapmx.get(&(key - k)).unwrap();
        //         let mn2 = mapmn.get(&(key - k)).unwrap();
        //         ans = ans.max(mx1 - mn2).max(mx2 - mn1);
        //     }
        // }
        if ans == i64::MIN {
            0_i64
        } else {
            ans
        }
    }
}



struct Solution {}

fn main()
{

    // let vi = [1,2,3,4,5,6].to_vec();
    // let k = 1;

    let vi = [-1,-2,-3,-4].to_vec();
    let k = 2;

    println!("ans: {:?}", Solution::maximum_subarray_sum(vi, k));
}


