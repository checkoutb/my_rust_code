



// pub fn satisfies_conditions(A: Vec<Vec<i32>>) -> bool {
//     (0..A.len() - 1).all(|i| (0..A[0].len()).all(|j| A[i][j] == A[i + 1][j])) &&
//     (0..A[0].len() - 1).all(|j| (0..A.len()).all(|i| A[i][j] != A[i][j + 1]))
// }


// grid[0].windows(2).all(|w| w[0] != w[1]) && grid.windows(2).all(|w| w[0] == w[1])



// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.04MB
// Beats100.00%of users with Rust
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        for i in 1..sz2 {
            if grid[0][i - 1] == grid[0][i] {
                return false;
            }
        }
        for i in 1..sz1 {
            for j in 0..sz2 {
                if grid[i][j] != grid[i - 1][j] {
                    return false;
                }
            }
        }
        true
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


