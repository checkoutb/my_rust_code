



// hint: Sort the array and try each value in range as a candidate.
// ..确实，不过可以更快。  计数数组， 前缀和， 
//      遍历 i in 0-max， 通过前缀和 可以计算出  i-k - i+k 范围 有多少个数字, 如果  <= k，那么 范围内的元素 都可以变成 i。 否则就只有 k个可以变成 i。
//      还要注意， [i] 是不需要变的 (不占用 k)  所以上面是 i-k 到 i+k 减去 [i]


// g

// max appear cnt
// [i]-k - [i]+k
// slide window
// [win.st] + k  >=  [win.en] - k

// num_ops...

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        
        let mut nums = nums;
        nums.sort();
        let mut st = 0usize;
        let mut ans = 0i32;
        // let mut stk = nums[st] + k;

        for i in 0..nums.len() {
            // while stk < nums[i]
            while nums[st] + k < nums[i] - k {
                st += 1;
            }
            ans = ans.max((i - st + 1) as i32);
        }

        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


