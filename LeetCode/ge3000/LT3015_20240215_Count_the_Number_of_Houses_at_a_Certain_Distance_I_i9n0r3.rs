






// too slow, most are < 10ms
// I is medium, II is hard

// Runtime
// 148ms
// Beats5.56%of users with Rust
// Memory
// 2.20MB
// Beats52.78%of users with Rust

// floyd

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        // if x == y {
        //     ans[0] = n * (n - 1);
        //     return ans;
        // }
        let mut vvi = vec![vec![i32::MAX; (n + 1) as usize]; (n + 1) as usize];
        for i in 2..=n as usize {
            vvi[i - 1][i] = 1;
            vvi[i][i - 1] = 1;
        }
        vvi[x as usize][y as usize] = 1;
        vvi[y as usize][x as usize] = 1;

        for k in 1..=n as usize {
            for i in 1..=n as usize {
                for j in 1..=n as usize {
                    if vvi[i][k] != i32::MAX && vvi[k][j] != i32::MAX && vvi[i][k] + vvi[k][j] < vvi[i][j] {
                        vvi[i][j] = vvi[i][k] + vvi[k][j];
                    }
                }
            }
        }

        // println!("{:?}", vvi);
        // for vi in &vvi {
        //     println!("{:?}", vi);
        // }

        for i in 1..=n as usize {
            for j in (i + 1)..=n as usize {
                if vvi[i][j] != i32::MAX {
                    ans[vvi[i][j] as usize - 1] += 2;
                }
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{
    // let (n,x,y) = (5,2,4);
    // let (n,x,y) = (3,1,3);
    let (n,x,y) = (7,1,7);

    println!("ans: {:?}", Solution::count_of_pairs(n,x,y));
}


