




// D D

// vec[vec[vec[0;16]]]
// 保存bit 而不是 值   不，就是值。  k<16, 不是 有16个bit。...



// Runtime
// 281ms
// Beats6.25%
// Memory
// 35.01MB
// Beats6.25%


impl Solution {
    pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::HashMap;
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        let mut vvmp = vec![vec![HashMap::new(); sz2]; sz1];

        vvmp[0][0].insert(grid[0][0], 1);
        const MOD: i32 = 1000000007;
        for i in 0..sz1 {
            for j in 0..sz2 {
                for (key, val) in vvmp[i][j].clone().into_iter() {
                    if i + 1 < sz1 {
                        let t2 = key ^ grid[i + 1][j];
                        vvmp[i + 1][j].entry(t2).and_modify(|v| *v = (*v + val) % MOD).or_insert(val);
                    }
                    if j + 1 < sz2 {
                        let t2 = key ^ grid[i][j + 1];
                        vvmp[i][j + 1].entry(t2).and_modify(|v| *v = (*v + val) % MOD).or_insert(val);
                    }
                }
            }
        }
        *vvmp[sz1 - 1][sz2 - 1].get(&k).unwrap_or(&0)
    }
}

struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


