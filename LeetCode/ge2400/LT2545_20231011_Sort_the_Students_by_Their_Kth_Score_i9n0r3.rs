





// Runtime21 ms
// Beats
// 20%
// Memory3 MB
// Beats
// 30%
impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score;
        score.sort_by_key(|a| -a[k as usize]);
        score
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


