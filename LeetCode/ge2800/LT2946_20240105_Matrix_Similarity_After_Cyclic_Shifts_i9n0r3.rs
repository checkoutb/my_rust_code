

// D D




// Runtime0 ms
// Beats
// 100%
// Memory2.2 MB
// Beats
// 23.53%

// 奇数 ->  ,,  偶数 <-
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let mut t2 = 1i32;
        let sz2 = mat[0].len() as i32;
        for i in 0..mat.len() {
            for j in 0..sz2 {
                if i % 2 == 0 {
                    t2 = 1;
                } else {
                    t2 = -1;
                }
                if mat[i][j as usize] != mat[i][(((j + t2 * k) % sz2 + sz2) % sz2) as usize] {
                    return false;
                }
            }
        }
        return true;
    }
}





struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


