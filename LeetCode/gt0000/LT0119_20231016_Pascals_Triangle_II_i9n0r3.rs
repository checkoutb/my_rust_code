




// Runtime0 ms
// Beats
// 100%
// Memory2.2 MB
// Beats
// 24.39%

// 0 [1]
// 1 [1,1]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut vi = vec![0; (row_index + 1) as usize];
        vi[0] = 1;
        for i in 0..((row_index) as usize) {
            let mut j = i + 1;
            while j > 0 {
                vi[j] += vi[j - 1];
                j -= 1
            }
        }
        vi
    }
}




struct Solution {}

fn main()
{

    let n = 3;

    println!("ans: {:?}", Solution::get_row(n));
}


