

// D D


// pub fn min_operations(nums: Vec<i32>) -> i32 {
//     const MAX: usize = 1_000_001;
//     lazy_static! {
//         static ref DIVISORS: Vec<i32> = {
//             let mut divisors = vec![1; MAX];
//             for num in 2..=MAX {
//                 let mut cur = num << 1;   // cur 是被除数,  最开始的时候 是 2倍于num， 后续while中 每次增加 num，就是 3倍，4倍...
//                 while cur < MAX {
//                     divisors[cur] = num as i32;  
//                     cur += num;   // 被除数 + num。     num是2->max，所以 divisors的值会越来越大。
//                 }
//             }
//             divisors
//         };
//     }
//
// DIVISORS[i] 就是 下标i的 最大除数。




// Runtime
// 49ms
// Beats68.25%
// Analyze Complexity
// Memory
// 3.59MB
// Beats53.97%



// [105, 11]  ...  .. 是计算 最小 factor 就可以了， 就是 factor中的 最小值。
// [9, 2] -> [3, 2]   3 is prime.  return -1

// 1000 000  1000

// no down   end -> begin
// how to get greatest proper divisor ... max factor.. so just / prime ?   keep all factor
// faster than O(nlogn)
impl Solution {



    pub fn min_operations(nums: Vec<i32>) -> i32 {
        
        let mut nums = nums;
        let mut idx1 = nums.len() - 1;  // idx - 1 is real index

        let mut vprime = vec![];
        let mut vb = vec![false; 1000];
        let mut ans = 0;

        for i in 2..vb.len() {
            if vb[i] == false {
                vprime.push(i as i32);
                let mut t2 = i + i;
                while t2 < vb.len() {
                    vb[t2] = true;
                    t2 += i;
                }
            }
        }

        while idx1 > 0 {
            
            if nums[idx1 - 1] > nums[idx1] {
                let mut t2 = nums[idx1 - 1];  // never 1
                
                let mut mnfactor = t2;

                for i in 0..vprime.len() {
                    if t2 % vprime[i] == 0 {
                        mnfactor = vprime[i];
                        break;
                    }
                }

                t2 = nums[idx1 - 1];
                if mnfactor == t2 {
                    return -1;  // nums[idx1-1] is prime, cannot reduce
                }
                if mnfactor > nums[idx1] {
                    return -1;
                }
                ans += 1;
                nums[idx1 - 1] = mnfactor;
            }

            idx1 -= 1;
        }
        ans
    }



    pub fn min_operations_errrrror(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut idx1 = nums.len() - 1;  // idx - 1 is real index

        let mut vprime = vec![];
        let mut vb = vec![false; 1000];
        let mut ans = 0;

        for i in 2..vb.len() {
            if vb[i] == false {
                vprime.push(i as i32);
                let mut t2 = i + i;
                while t2 < vb.len() {
                    vb[t2] = true;
                    t2 += i;
                }
            }
        }

        while idx1 > 0 {
            
            if nums[idx1 - 1] > nums[idx1] {
                let mut vfactor = vec![];
                let mut t2 = nums[idx1 - 1];  // never 1
                for i in 0..vprime.len() {
                    if t2 == 1 {
                        break;
                    }
                    while t2 % vprime[i] == 0 {
                        vfactor.push(vprime[i]);
                        t2 /= vprime[i];
                    }
                }
                if t2 != 1 {
                    vfactor.push(t2);
                }

                vfactor.reverse();

                t2 = nums[idx1 - 1];
                let mut j = 0usize;
                if vfactor[j] == t2 {
                    return -1;  // nums[idx1-1] is prime, cannot reduce
                }
                while t2 > nums[idx1] {
                    if vfactor[j] == t2 {
                        return -1;  // nums[idx1-1] is prime, cannot reduce
                    }
                    t2 /= vfactor[j];
                    j += 1;
                    ans += 1;
                }
                nums[idx1 - 1] = t2;
            }


            idx1 -= 1;
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


