

// D D

// const NEIGHBOUR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

//         let mut heap = BinaryHeap::from([(health, 0_usize, 0_usize)]);
//         while let Some((h, r, c)) = heap.pop() {
//             NEIGHBOUR.iter().for_each(|&(m, n)| {
//                 let (a, b) = (r as i32 + m, c as i32 + n);
//                 if !(0 <= a && a < row as i32 && 0 <= b && b < col as i32) {
//                     return;
//                 }
//                 heap.push((new_distance, a, b));
//             });
//         }


// Runtime
// 7ms
// Beats91.30%
// Analyze Complexity
// Memory
// 2.31MB
// Beats8.70%


impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {

        use std::collections::BinaryHeap;

        let sz1 = grid.len();
        let sz2 = grid[0].len();
        let mut vvi = vec![vec![0; sz2]; sz1];  // max health

        let mut priq = BinaryHeap::new();
        vvi[0][0] = health - grid[0][0];
        const MUL1:i32 = 10000i32;
        const MUL2:i32 = 100i32;
        priq.push(vvi[0][0] * MUL1);

        let arr = [-1,0,1,0,-1].to_vec();

        while !priq.is_empty() {
            let t2 = priq.pop().unwrap();
            let hp = t2 / MUL1;
            let x = (t2 % MUL1) / MUL2;
            let y = t2 % MUL2;
            if x == sz1 as i32 - 1 && y == sz2 as i32 - 1 {
                return true;
            }

            if hp != vvi[x as usize][y as usize] {
                continue;
            }

            for i in 1..arr.len() {
                let nx = x + arr[i - 1];
                let ny = y + arr[i];
                if nx < 0 || ny < 0 || nx == sz1 as i32 || ny == sz2 as i32 {
                    continue;
                }
                let nhp = hp - grid[nx as usize][ny as usize];
                if nhp <= vvi[nx as usize][ny as usize] {
                    continue;
                }

                vvi[nx as usize][ny as usize] = nhp;
                priq.push(nhp * MUL1 + nx * MUL2 + ny);
            }
        }

        false
    }
}



struct Solution {}

fn main()
{

    let vvi = [[1,1,1].to_vec(), [1,0,1].to_vec(), [1,1,1].to_vec()].to_vec();
    let hp = 4;

    println!("ans: {:?}", Solution::find_safe_walk(vvi, hp));
}


