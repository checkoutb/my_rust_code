


// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.36MB
// Beats100.00%




/*

2个机器人不会相交。

所以 [i][j]  robot1在 当前行的 i列,  robot2在 当前行的 j列  的最多采集。

然后每次是 for(0 - sz1-2) { for (isz1-1 - i) }

每个点 3个 来源。


*/


impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let sz1 = grid.len();
        let sz2 = grid[0].len();

        let mut vvi = vec![vec![-1; sz2]; sz2];
        vvi[0][sz2 - 1] = grid[0][0] + grid[0][sz2 - 1];

        let arr = [-1,0,1].to_vec();

        // println!("{:?}", grid);

        for i in 1..sz1 {
            let mut vv2 = vec![vec![-1; sz2]; sz2];

            for j1 in 0..sz2 {
                if j1 > i {
                    break;
                }
                let mut j2 = sz2 - 1;
                while j2 > j1 {
                    let mut t2 = -1;
                    
                    for a in 0..3 {
                        for b in 0..3 {
                            let oi = j1 as i32 + arr[a];
                            let oj = j2 as i32 + arr[b];

                            if oi < 0 || oi >= oj || oj >= sz2 as i32 {
                                continue;
                            }
                            if vvi[oi as usize][oj as usize] == -1 {  // .......
                                continue;
                            }
                            t2 = t2.max(vvi[oi as usize][oj as usize]);
                        }
                    }
                    if t2 == -1 {   // ......
                        break;
                    }
                    vv2[j1][j2] = t2 + grid[i][j1] + grid[i][j2] ;
                    
                    j2 -= 1;
                }
            }

            // println!("{:?}", vv2);
            // println!("==========");

            vvi = vv2;
        }

        let mut ans = 0;

        for i in 0..sz2 {
            for j in 0..sz2 {
                ans = ans.max(vvi[i][j])
            }
        }

        ans
    }
}



struct Solution {}

fn main()
{

    // let vvi = [[3,1,1].to_vec(),[2,5,1].to_vec(),[1,5,5].to_vec(),[2,1,1].to_vec()].to_vec();

    // 96
    let vvi = [[0,8,7,10,9,10,0,9,6].to_vec(),[8,7,10,8,7,4,9,6,10].to_vec(),
    [8,1,1,5,1,5,5,1,2].to_vec(),[9,4,10,8,8,1,9,5,0].to_vec(),
    [4,3,6,10,9,2,4,8,10].to_vec(),[7,3,2,8,3,3,5,9,8].to_vec(),
    [1,2,6,5,6,2,0,10,0].to_vec()].to_vec();

    println!("ans: {:?}", Solution::cherry_pickup(vvi));
}


