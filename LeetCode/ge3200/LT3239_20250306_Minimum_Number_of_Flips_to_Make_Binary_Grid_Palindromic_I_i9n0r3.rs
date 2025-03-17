










// Runtime
// 0ms
// Beats100.00%
// Memory
// 15.52MB
// Beats-%



impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        
        // row
        let mut chg = 0;
        let mut ans = 0;
        for i in 0..sz1 {
            for j in 0..(sz2 / 2) {
                if grid[i][j] != grid[i][sz2 - 1 - j] {
                    chg += 1;
                }
            }
        }
        ans = chg;

        // column
        chg = 0;
        for j in 0..sz2 {
            for i in 0..(sz1 / 2) {
                if grid[i][j] != grid[sz1 - 1 - i][j] {
                    chg += 1;
                }
            }
        }

        ans.min(chg)
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


