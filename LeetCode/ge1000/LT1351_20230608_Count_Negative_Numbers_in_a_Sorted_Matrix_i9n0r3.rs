

// D D

// grid.iter().map(|x| x.iter().filter(|y| y < &&0).count() as i32).sum()



// Runtime2 ms
// Beats
// 78.79%
// Memory2.4 MB
// Beats
// 12.12%
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;
        let mut j = grid[0].len() as i32 - 1;
        for i in 0..grid.len() {
            while j != -1 && grid[i][j as usize] < 0 {
                j -= 1
            }
            ans += (grid[0].len()) as i32 - j - 1;
        }
        ans
    }
}





struct Solution {}

fn main()
{
    let vvi = [[4,3,2,-1].to_vec(),[3,2,1,-1].to_vec(),[1,1,-1,-2].to_vec(),[-1,-1,-2,-3].to_vec()].to_vec();

    println!("ans: {:?}", Solution::count_negatives(vvi));
}


