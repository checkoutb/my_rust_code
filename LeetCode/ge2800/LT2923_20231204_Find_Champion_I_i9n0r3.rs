






// Runtime10 ms
// Beats
// 20.24%
// Memory2.3 MB
// Beats
// 8.33%

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let sz1 = grid.len();

        'AAA: for j in 0..sz1 {
            for i in 0..sz1 {
                if grid[i][j] == 1 {
                    continue 'AAA;
                }
            }
            return j as i32;
        }
        -1
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


