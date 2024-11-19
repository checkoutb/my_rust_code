



// D D

// let mut counter = [0i64; 24];


// +=



// Runtime
// 14ms
// Beats82.43%
// Analyze Complexity
// Memory
// 6.16MB
// Beats98.65%
impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut vi = vec![0i64; 24];
        let mut ans = 0i64;
        let mut t2 = 0usize;
        for h in hours.iter() {
            t2 = (h % 24) as usize;
            vi[t2] = vi[t2] + 1;
        }
        ans = (1 + vi[0] - 1) * (vi[0] - 1) / 2 + (1 + vi[12] - 1) * (vi[12] - 1) / 2; // 0, 12
        for i in 1..12 {
            ans = ans + (vi[i] * vi[24 - i]);
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


