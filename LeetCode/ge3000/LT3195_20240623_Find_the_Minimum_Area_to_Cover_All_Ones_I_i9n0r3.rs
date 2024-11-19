








// Runtime
// 40ms
// Beats100.00%
// Analyze Complexity
// Memory
// 8.84MB
// Beats100.00%

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut mni = usize::MAX;
        let mut mnj = usize::MAX;
        let mut mxi = 0usize;
        let mut mxj = 0usize;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    mni = mni.min(i);
                    mnj = mnj.min(j);
                    mxi = mxi.max(i);
                    mxj = mxj.max(j);
                }
            }
        }
        ((mxi - mni + 1) * (mxj - mnj + 1)) as i32
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


