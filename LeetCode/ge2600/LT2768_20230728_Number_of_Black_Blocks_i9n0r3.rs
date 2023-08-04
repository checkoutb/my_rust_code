






// g



// m*n <= 10^10....
// points.length <= 10^4
// sort by points's row then by col
// 2 vector, 2 pointer. 
// check when @ bottom-right ... no .. 
impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        let mut coordinates = coordinates;
        coordinates.sort();
        let n: usize = n as usize;
        let m: usize = m as usize;
        let mut ans = vec![0 as i64; 5];
        let mut vvi = vec![vec![]; m];
        for vi in &coordinates {
            vvi[vi[0] as usize].push(vi[1] as usize);
        }

        // println!("{:?}", &coordinates);
        // // println!("{:?}", &vvi);
        // for vi in &vvi {
        //     println!("{:?}", vi);
        // }

        for i in 0 as usize..m as usize {
            let mut pi = 0;
            let mut ni = 0;
            for j in 0..vvi[i].len() {
                if i > 0 {
                    while pi < vvi[i - 1].len() && vvi[i - 1][pi] < vvi[i][j] {
                        pi += 1;
                    }
                    let mut t2 = 1;
                    if j > 0 && vvi[i][j - 1] + 1 == vvi[i][j] {
                        t2 += 1;
                    }
                    if pi < vvi[i - 1].len() && vvi[i - 1][pi] == vvi[i][j] {
                        t2 += 1;
                    }
                    if pi > 0 && vvi[i - 1][pi - 1] + 1 == vvi[i][j] {
                        t2 += 1;
                    }
                    ans[t2] += 1;

                    // println!(".... {}, {}", t2, pi);

                    if (j + 1 == vvi[i].len() || vvi[i][j + 1] != vvi[i][j] + 1) && vvi[i][j] != n - 1 {
                        t2 = 1;
                        if pi < vvi[i - 1].len() && vvi[i - 1][pi] == vvi[i][j] {
                            t2 += 1;
                            if pi + 1 < vvi[i - 1].len() && vvi[i - 1][pi + 1] == vvi[i][j] + 1 {
                                t2 += 1;
                            }
                        } else {
                            if pi < vvi[i - 1].len() && vvi[i - 1][pi] == vvi[i][j] + 1 {
                                t2 += 1;
                            }
                        }
                        ans[t2] += 1;
                    }
                }

                // println!("{:?} 111111- {} - {}", ans, i, j);

                if i < m - 1 {
                    while ni < vvi[i + 1].len() && vvi[i + 1][ni] < vvi[i][j] {
                        ni += 1;
                    }
                    if ni < vvi[i + 1].len() && vvi[i + 1][ni] == vvi[i][j] {
                        continue;
                    }
                    if ni == 0 || vvi[i + 1][ni - 1] + 1 != vvi[i][j] {
                        // if vvi[i][j] != 0 {
                            if j == 0 || vvi[i][j - 1] + 1 != vvi[i][j] {
                                if vvi[i][j] > 0 {
                                    // println!("11111111");
                                    ans[1] += 1;
                                }
                            }
                        // }
                    }
                    // println!("{}    ninini", ni);
                    if ni == vvi[i + 1].len() || vvi[i + 1][ni] != vvi[i][j] + 1 {
                        if j + 1 == vvi[i].len() || vvi[i][j + 1] != vvi[i][j] + 1 {
                            if vvi[i][j] < n - 1 {
                                // println!("2222222");
                                ans[1] += 1;
                            }
                        }
                    }

                    // if ni < vvi[i + 1].len() {
                    //     if vvi[i + 1][ni] == j {
                    //         ;
                    //     } else {
                    //         if vvi[i + 1][ni] - 1 == j {
                    //             ;
                    //         } else {
                    //             if j + 1 < vvi[i].len() && vvi[i][j + 1] - 1 == j {
                    //                 ;
                    //             }
                    //         }
                    //     }
                    // } else {

                    // }
                }

                // if i == 1 && j == 0 {
                    // println!("{:?} - {} - {}", ans, i, j);
                // }
            }
        }
        ans[0] = (m - 1) as i64 * (n - 1) as i64 - ans[1] - ans[2] - ans[3] - ans[4];
        ans


        // coordinates.sort();
        // let mut vvi = vec![vec![coordinates[0][0]]; 1];     // 1st is row, followers are col
        // let mut idx = 0;
        // for i in 0..coordinates.len() {
        //     if coordinates[i][0] == vvi[idx][0] {
        //         vvi[idx].push(coordinates[i][1]);
        //     } else {
        //         vvi.push(vec![]);
        //         ++idx;
        //         vvi[idx].push(coordinates[i][0]);
        //         vvi[idx].push(coordinates[i][1]);
        //     }
        // }
        // let mut ans = vec![0; 5];
        // if vvi.size() == 1 {
        //     if vvi[0][0] != 0 {
        //         ++ans[1];
        //     }
        //     for j in 1..vvi[0].size() {
        //         if vvi[0][j] == vvi[0][j - 1] + 1 {
        //             ++ans[2];
        //         } else {
        //             ++ans[1];
        //         }
        //     }
        // } else {
        //     for i in 1..vvi.size() {
        //         let mut j1 = 1;
        //         let mut j2 = 1;
        //         while j1 < vvi[i - 1].len() || j2 < vvi[i].len() {
        //             // .. tai ma fan le...
        //         }
        //         // let mut pi = 1;     // previous row
        //         // // let mut ni = 1;
        //         // let pb = i != 0 && vvi[i][0] == vvi[i - 1][0] + 1;
        //         // // let nb = i != vvi.size() - 1 && vvi[i][0] == vvi[i + 1] - 1;
        //         // for j in 1..vvi[i].size() {
                    
        //         // }
        //     }
        // }




        // coordinates.sort();
        // // let vi = vec![0; 0];
        // let mut vi = vec![];
        // let mut idx = 0;
        // let sz1 = coordinates.len();
        // while idx < sz1 && coordinates[idx][0] == 0 {
        //     vi.push(coordinates[idx][1]);
        //     ++idx;
        // }
        // let mut ans = vec![0; 5];
        // for i in 1..m {
        //     let mut j = 0;
        //     let mut v2 = vec![];
        //     while idx < sz1 && coordinates[idx][0] == i {
        //         v2.push(coordinates[idx][1]);
        // need check twice.. if now is empty, but upper is black..
        //     }
        //     vi = v2;
        // }

        // ans
    }
}



struct Solution {}

fn main()
{

    // let m = 3;
    // let n = 3;
    // // let vi = [[0,0].to_vec()].to_vec();
    // let vi = [[0,0].to_vec(),[1,1].to_vec(),[0,2].to_vec()].to_vec();

    // let m = 32;
    // let n = 32;     // [866,94,1,0,0]
    // // let vi = [[17,29].to_vec(),[29,16].to_vec(),[19,20].to_vec(),[18,9].to_vec(),[16,7].to_vec(),
    // // [20,25].to_vec(),[22,19].to_vec(),[4,9].to_vec(),[14,17].to_vec(),[6,23].to_vec(),[2,2].to_vec(),[20,1].to_vec(),[8,7],[4,7],[14,14],[10,10],[1,27],[18,23],[6,30],[8,18],[26,23],[25,8],[5,6],[3,4]].to_vec();

    // let vi = [[17,29].to_vec(),[29,16].to_vec(),[19,20].to_vec(),[18,9].to_vec(),
    // [16,7].to_vec(),[20,25].to_vec(),[22,19].to_vec(),[4,9].to_vec(),[14,17].to_vec(),
    // [6,23].to_vec(),[2,2].to_vec(),[20,1].to_vec(),[8,7].to_vec(),[4,7].to_vec(),
    // [14,14].to_vec(),[10,10].to_vec(),[1,27].to_vec(),[18,23].to_vec(),[6,30].to_vec(),
    // [8,18].to_vec(),[26,23].to_vec(),[25,8].to_vec(),[5,6].to_vec(),[3,4].to_vec()].to_vec();


    let m = 22;
    let n = 73;

    let vi = [[11,14],[16,11],[20,5],[5,33],[14,7],[16,60],[0,15],[15,72],[6,60],[9,16],[14,51],[1,52],[18,24],[17,30],[3,4],[19,13],[9,10],[11,40],[15,7],[13,62],[8,41],[12,71],[4,72],[18,7],[1,0],[4,35],[16,33],[7,30],[13,52],[5,1],[15,21],[3,59],[2,41],[4,28]];


    println!("ans: {:?}", Solution::count_black_blocks(m, n, vi));
}


