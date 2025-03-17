








// Runtime
// 691ms
// Beats55.56%
// Memory
// 4.06MB
// Beats88.89%






// at most 1  90 顺时针
// 1 2020202020

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        let mut ans = 0i32;
        for i in 0..sz1 {
            for j in 0..sz2 {
                if grid[i][j] == 1 {
                    for dir in 1..=4 {
                        ans = ans.max(1 + Self::dfsa1(&grid, i as i32, j as i32, dir as i32, 2));
                    }
                }
            }
        }
        ans
    }

    // dir: 1234 象限,  负1234 不能再转向
    // return tail length
    fn dfsa1(grid: &Vec<Vec<i32>>, mut pi: i32, mut pj: i32, dir: i32, tar: i32) -> i32 {
        // pi: parent's i
        match dir.abs() {

            // 象限和二位数组 不是很匹配。
            
            1 => { pi -= 1; pj += 1; },
            2 => { pi -= 1; pj -= 1; },
            3 => { pi += 1; pj -= 1; },
            4 => { pi += 1; pj += 1; },
            _ => unreachable!(),
        }

        if pi < 0 || pj < 0 || pi >= grid.len() as i32 || pj >= grid[0].len() as i32 {
            return 0;
        }
        if grid[pi as usize][pj as usize] != tar {
            return 0;
        }

        let mut ans = Self::dfsa1(grid, pi, pj, dir, 2 - tar);
        if dir > 0 {
            ans = ans.max(Self::dfsa1(grid, pi, pj, if dir == 1 { -4 } else { -(dir - 1) }, 2 - tar));
        }
        ans + 1
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


