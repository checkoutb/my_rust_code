









// Runtime
// 96ms
// Beats100.00%
// Memory
// 7.80MB
// Beats100.00%



// index,    == value   prefix same value,  suffix same value

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {

        use std::collections::HashMap;
        let mut mp2 = HashMap::new();
        
        let sz1 = nums.len();
        let mut vi = vec![-1; sz1];

        for i in 0..(sz1 * 2) {
            let t2 = nums[i % sz1];
            if let Some(idx) = mp2.get(&t2) {
                if idx % sz1 != i % sz1 {       // .........
                    vi[i % sz1] = (i - idx) as i32;
                }
            }
            mp2.insert(t2, i);
        }

        mp2.clear();
        for i in (0..(sz1 * 2)).rev() {
            let t2 = nums[i % sz1];
            if let Some(idx) = mp2.get(&t2) {
                if idx % sz1 != i % sz1 {
                    vi[i % sz1] = ((idx - i) as i32).min(vi[i % sz1]);
                }
            }
            mp2.insert(t2, i);
        }

        let mut v2 = vec![0; queries.len()];

        for i in 0..queries.len() {
            v2[i] = vi[queries[i] as usize];
        }

        v2
    }
}


struct Solution {}

fn main()
{

    let vi = [1,3,1,4,1,3,2].to_vec();
    let q = [0,3,5].to_vec();

    println!("ans: {:?}", Solution::solve_queries(vi, q));
}


