





// g， 出现 相同值时， 需要判断 下标， 这个 太麻烦了。。 方向不对。


// 是指 下标组成的数组 的 字典序最小？

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        
        // use std::collections::BinaryHeap;

        // let mut intervals = intervals;

        // let mut priq: BinaryHeap<(i32, i32)> = BinaryHeap::new();

        // intervals.sort();


        let mut intervals = intervals;
        for i in 0..intervals.len() {
            intervals[i].push(i as i32);
        }
        // intervals.sort_by_key(|e| e[1]);
        intervals.sort_by(|a, b| if a[1] == b[1] { b[3].cmp(&a[3]) } else { a[1].cmp(&b[1]) });

        let mut vvi = vec![vec![vec![]; 4]; intervals.len()];
        // vvi[0][0] = {intervals[0][2], intervals[0][2]};
        vvi[0][0].push(intervals[0][2] as i64);
        vvi[0][0].push(intervals[0][3] as i64);

        for i in 1..intervals.len() {

            let t2 = Self::finda1(&intervals, intervals[i][0] - 1, i);

            if t2 != -1 {
                let t2 = t2 as usize;
                let wgh = intervals[i][2];
                for j in 1..4 {

                    if vvi[t2][j - 1].is_empty() {
                        vvi[i][j] = vvi[i - 1][j].clone();
                        continue;
                    }

                    if vvi[i - 1][j].is_empty() || wgh as i64 + vvi[t2][j - 1][0] as i64 > vvi[i - 1][j][0] {
                        vvi[i][j] = vvi[t2][j - 1].clone();
                        // vvi[i][j].push(wgh);
                        vvi[i][j].push(intervals[i][3] as i64);
                        vvi[i][j][0] = wgh as i64 + vvi[t2][j - 1][0] as i64;
                    } else {
                        vvi[i][j] = vvi[i - 1][j].clone();
                    }
                }
            } else {
                for j in 1..4 {
                    if vvi[i - 1][j].is_empty() {
                        break;
                    }
                    vvi[i][j] = vvi[i - 1][j].clone();
                }
                // if intervals[i][2] > vvi[i][0][0] {
                //     vvi[i][0][0] = intervals[i][2];
                //     vvi[i][0][1] = intervals[i][3];
                // }
            }

            if intervals[i][2] as i64 > vvi[i - 1][0][0] {
                // vvi[i][0] = {intervals[i][2], intervals[i][2]};
                vvi[i][0].push(intervals[i][2] as i64);
                vvi[i][0].push(intervals[i][3] as i64);
            } else {
                vvi[i][0] = vvi[i - 1][0].clone();
                
            }
        }

        for i in 0..intervals.len() {
            println!("{:?}", intervals[i]);
        }
        println!(" ========== ");

        for i in 0..vvi.len() {
            println!("1: {:?}, 2: {:?},  3: {:?}, 4: {:?}", vvi[i][0], vvi[i][1], vvi[i][2], vvi[i][3]);
        }


        let mut vi: Vec<i64> = vec![];
        let mut ans = vec![];
        let mut t2 = vec![];
        let mut mx = -1;
        for i in 0..4 {
            if !vvi[vvi.len() - 1][i].is_empty() && vvi[vvi.len() - 1][i][0] > mx {
                mx = vvi[vvi.len() - 1][i][0];
            }
        }
        for i in 0..4 {
            if !vvi[vvi.len() - 1][i].is_empty() {
                // if vi.is_empty() {
                //     vi = vvi[vvi.len() - 1][i].clone();
                // } else if vi[0] <=

                if !vvi[vvi.len() - 1][i].is_empty() && vvi[vvi.len() - 1][i][0] == mx {
                    // vi = vvi[vvi.len() - 1][i].clone();
                    // t2 = vec![];
                    // for i in 0..intervals.len() {
                    //     for j in 1..vi.len() {
                    //         if intervals[i][2] == vi[j] {
                    //             t2.push(intervals[i][3]);
                    //             vi[j] = -1;
                    //             break;
                    //         }
                    //     }
                    // }

                    t2 = vvi[vvi.len() - 1][i][1..].to_vec();
                    t2.sort();
                    if ans.is_empty() || ans > t2 {
                        ans = t2;
                    }
                }
            }
        }

        let mut ans2 = vec![];
        for i in 0..ans.len() {
            ans2.push(ans[i] as i32);
        }
        ans2


        // println!("{:?}", vi);

        // for i in 0..intervals.len() {
        //     for j in 1..vi.len() {
        //         if intervals[i][2] == vi[j] {
        //             ans.push(i as i32);
        //             vi[j] = -1;
        //             break;
        //         }
        //     }
        // }

        // ans
    }

    fn finda1(intervals: &Vec<Vec<i32>>, tar: i32, en: usize) -> i32 {
        let mut st = 0;
        let mut en = en;
        let mut ans = -1;
        while st <= en {
            let md = (st + en) / 2;
            if intervals[md][1] <= tar {
                st = md + 1;
                ans = md as i32;
            } else {
                if md == 0 {
                    break;
                }
                en = md - 1;
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{

    // let vvi = [[1,3,2].to_vec(),[4,5,2].to_vec(),[1,5,5].to_vec(),[6,9,3].to_vec(),[6,7,1].to_vec(),[8,9,1].to_vec()].to_vec();

    // let vvi = [[5,8,1].to_vec(),[6,7,7].to_vec(),[4,7,3].to_vec(),[9,10,6].to_vec(),[7,8,2].to_vec(),[11,14,3].to_vec(),[3,5,5].to_vec()].to_vec();

    // let vvi = [[23,23,15].to_vec(),[21,21,55].to_vec(),[12,15,22].to_vec()].to_vec();

    // let vvi = [[7,16,26].to_vec(),[23,23,15].to_vec(),[1,15,34].to_vec(),[20,20,50].to_vec(),[12,17,45].to_vec(),[7,23,15].to_vec(),[19,24,30].to_vec(),[2,24,27].to_vec(),[16,24,7].to_vec(),[14,21,3].to_vec()].to_vec();

    let vvi = [[8,15,32].to_vec(),[20,21,8].to_vec(),[8,16,29].to_vec(),[7,12,50].to_vec(),[16,25,27].to_vec(),[12,17,2].to_vec(),[8,12,45].to_vec(),[5,10,50].to_vec()].to_vec();

    println!("ans: {:?}", Solution::maximum_weight(vvi));
}


