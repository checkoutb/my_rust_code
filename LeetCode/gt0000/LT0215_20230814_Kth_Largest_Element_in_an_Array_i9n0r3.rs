



// D D


// pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//     *nums.select_nth_unstable_by_key(k as usize - 1, |&x| Reverse(x)).1
// }

// nums.swap(len-1,len/2);

// let count = |target: i32| -> i32 {
//     let mut res = 0;
//     for num in nums.iter() {
//         if num <= &target {
//             res += 1;
//         }
//     }
//     res
// };
// while l < r {
//     let m = l + (r - l) / 2;
//     // find smallest m s.t. k elements <= target
//     if count(m) >= k {
//         r = m;
//     } else {
//         l = m + 1;
//     }
// }
// l


impl Solution {

    // Runtime19 ms
    // Beats
    // 45.92%
    // Memory3.1 MB
    // Beats
    // 46.94%
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums;
        let en = nums.len() - 1;
        return Self::dfsa1(nums, k as usize, 0, en);
    }

    fn dfsa1(mut nums: Vec<i32>, k: usize, st: usize, en: usize) -> i32 {
        if st == en {
            return nums[st];
        }
        let flg = nums[en];
        let mut fst = st;
        let mut lst = en;   // position
        let mut idx = st;
        while idx < lst {
            if nums[idx] > flg {
                // let t2 = nums[idx];
                nums[lst] = nums[idx];
                nums[idx] = nums[lst - 1];
                lst -= 1;
            } else {
                fst += 1;
                idx += 1;
            }
        }
        nums[lst] = flg;

        if lst == nums.len() - k {
            return nums[lst];
        } else if lst > nums.len() - k {
            return Self::dfsa1(nums, k, st, lst - 1);
        } else {
            return Self::dfsa1(nums, k, lst + 1, en);
        }
        
        // for i in st..en {
        //     if nums[i] > flg {

        //     }
        // }
    }


    // Runtime21 ms
    // Beats
    // 38.27%
    // Memory3.2 MB
    // Beats
    // 25%
    pub fn find_kth_largest_______1(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        nums[nums.len() - k as usize]
    }
}





struct Solution {}

fn main()
{

    let vi = [3,2,1,5,4,6].to_vec();
    let k = 2;

    println!("ans: {:?}", Solution::find_kth_largest(vi, k));
}


