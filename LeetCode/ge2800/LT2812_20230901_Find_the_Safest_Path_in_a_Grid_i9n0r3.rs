



// unfinish


// yuanli thief. max { min dis to thief in path }
// bfs
// bs
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut que = std::collections::VecDeque::new();
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        for i in 0..sz1 {
            for j in 0..sz2 {
                if grid[i][j] == 1 {
                    que.push_back((i, j, 0));       // i,j,length to thief
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = i32::MAX;
                }
            }
        }
        let dir = [1,0,-1 as i32,0,1].to_vec();
        while !que.is_empty() {
            let (i, j, dis) = que.pop_front().unwrap();
            for k in 1..5 {
                let ni = i + dir[k - 1];
                let nj = j + dir[k];
                if ni >= 0 && nj >= 0 && ni < sz1 && nj < sz2 && grid[ni][nj] > dis + 1 {
                    grid[ni][nj] = dis + 1;
                    que.push_back((ni, nj, dis + 1));
                }
            }
        }
        let mut st = 0;
        let mut en = (sz1 + sz2) as i32;
        let mut ans = 0;
        while st <= en {
            let md = (st + en) / 2;
            if Self::can_reach(grid, md) {
                ans = md;
                st = md + 1;
            } else {
                en = md - 1;
            }
        }
        ans
    }

    // >=mx is good
    fn can_reach(vvi: Vec<Vec<i32>>, mx: i32) -> bool {
        if vvi[0][0] < mx {
            return false;
        }
        let mut que = std::collections::VecDeque::new();
        que.push_back((0, 0));
        vvi[0][0] = mx - 1;
        let dir = [1,0,-1 as i32,0,1].to_vec();
        while !que.is_empty() {
            let (i, j) = que.pop_front().unwrap();
            if i == vvi.len() - 1 && j == vvi[0].len() - 1 {
                return true;
            }
            for k in 1..5 {
                let ni = i as i32 + dir[k - 1];
                let nj = j as i32 + dir[k];
                if ni >= 0 && nj >= 0 && (ni as usize < vvi.len()) && (nj as usize < vvi[0].len()) && vvi[ni][nj] >= mx {
                    que.push_back((ni, nj));
                    vvi[ni as usize][nj as usize] = mx - 1;
                }
            }
        }
        return false;
    }
}

// let mut queue = std::collections::VecDeque::new();
// queue.push_back((i, j, 0 as i32));
// let (x, y, level) = queue.pop_front().unwrap();
// while !queue.is_empty() {


struct Solution {}

fn main()
{

    let vvi = [[0,0,1].to_vec(),[0,0,0].to_vec(),[0,0,0].to_vec()].to_vec();

    println!("ans: {:?}", Solution::maximum_safeness_factor(vvi));
}


