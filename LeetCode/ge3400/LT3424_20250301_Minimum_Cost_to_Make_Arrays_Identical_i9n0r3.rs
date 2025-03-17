



// Runtime
// 46ms
// Beats25.00%
// Memory
// 4.16MB
// Beats75.00%


// 都 sort 然后for？

impl Solution {
    pub fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        
        let mut ans = 0i64;
        let mut t2 = 0i64;
        for i in 0..arr.len() {
            t2 += (arr[i] - brr[i]).abs() as i64;
        }
        ans = t2;

        let mut arr = arr;
        let mut brr = brr;
        arr.sort();
        brr.sort();

        t2 = k;
        for i in 0..arr.len() {
            t2 += (arr[i] - brr[i]).abs() as i64;
        }

        ans = ans.min(t2);

        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


