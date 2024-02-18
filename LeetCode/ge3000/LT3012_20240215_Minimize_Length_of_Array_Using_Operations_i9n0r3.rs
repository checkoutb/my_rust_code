

// D D

// let min_num = *nums.iter().reduce(|acc, e| std::cmp::min(acc, e)).unwrap();
// let min_num_count = nums.iter().filter(|&&x| x == min_num).count() as i32;
// if min_num_count <= 2 || nums.iter().any(|&x| x % min_num != 0) {
//     1
// } else {
//     (min_num_count + 1) / 2
// }


// Runtime
// 15ms
// Beats21.43%of users with Rust
// Memory
// 4.02MB
// Beats51.79%of users with Rust


// x<y
// x,y,y+a,y+b,y+c...y+z    x%(y+{any>=0})=x   min size is 1
// x,x,y,y+a...    x%x=0   min size is 1
// x,x,x,y,y+a...   2 x,0
// x,x,x,x,y...     2 0,0
// x,x,x,x,x,y...   3

// ... error

// 2,2,2,5,9,10  ->  1    5%2=1   then vec become:  1,2,2,9,10   .. min element is 1

// 3,3,3,4,  4%3=1

// (n+1) % n = 1
// (2n+1) % n = 1
// (xn+y) % n = y

// 3,3,3,3,5,123,123,123  ->1   5%3 = 2, min element is 2

// just use min element to % others,   other % min < min   , if find an index i that nums[i] % min != 0, then min element's count is 1

// 5,5,5,5,5, a,b,c,....z    if cannot find an index i that nums[i] % 5 != 0， means a,b,c...z are 5's multiple
//                                                              means cannot find an index i that nums[i] % nums[j] != 0
//          so just use min element to % others to find if there is an nums[i] % min != 0

impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = 0;
        let t2 = nums[0];
        for i in nums {
            if i == t2 {
                cnt += 1;
            } else {
                if i % t2 != 0 {
                    return 1;
                }
            }
        }
        (cnt + 1) / 2
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


