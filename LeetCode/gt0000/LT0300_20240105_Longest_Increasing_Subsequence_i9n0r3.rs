




// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let mut end_of_longest_seq = vec![i32::MAX; nums.len() + 1];
//         for (index, num) in nums.into_iter().enumerate() {
//             let point = end_of_longest_seq.partition_point(|&x| x < num);
//             end_of_longest_seq[point] = end_of_longest_seq[point].min(num);
//         }
//         end_of_longest_seq.partition_point(|&x| x < i32::MAX) as i32
//     }
// }



// impl Solution {
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         let n = nums.len();
//         let mut dp = vec![1; n];
//         let mut result = 1;
//
//         for i in 1..n {
//             for j in 0..i{
//                 if nums[j] < nums[i] {
//                     dp[i] = std::cmp::max(dp[i], dp[j] + 1);
//                 }
//             }
//             result = std::cmp::max(result, dp[i]);
//         }
//
//         result        
//     }
// }





impl Solution {
    // use xxxHashMap .. error, must in funcrion

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        // let mut vi = vec![0; nums.len()];
        let mut map2: HashMap<i32, i32> = HashMap::new();
        let mut t2 = 0i32;
        // for num in nums.iter() {
        for num in nums {
            t2 = 0;
            for (k, v) in map2.clone().into_iter() {
                if k < num {
                    t2 = t2.max(v);
                }
            }
            t2 += 1;
            map2.entry(num).and_modify(|e| *e = t2.max(*e)).or_insert(t2);
        }

        map2.into_values().max().unwrap()
    }
}



struct Solution {}

fn main()
{

    // let vi = [10,9,2,5,3,7,101,18].to_vec();
    // let vi = [0,1,0,3,2,3].to_vec();
    let vi = [7,7,7,6].to_vec();

    println!("ans: {:?}", Solution::length_of_lis(vi));
}


