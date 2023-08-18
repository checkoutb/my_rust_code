
// D D


// let (m, n) = (mat.len(), mat[0].len());
// let mut res_mat = vec![vec![i32::MAX; n]; m];
// let mut queue = VecDeque::new();
// for i in 0..m {
//     for j in 0..n {
//         if mat[i][j] == 0 {
//             res_mat[i][j] = 0;
//             queue.push_back((i, j))
//         }
//     }
// }
// while let Some((i, j)) = queue.pop_front() {
//     // sugyan ingenuity
//     for d in [0, 1, 0, !0, 0].windows(2) {
//         let di = i.wrapping_add(d[0]);
//         let dj = j.wrapping_add(d[1]);
//         if di < m && dj < n && res_mat[di][dj] > res_mat[i][j] {
//             res_mat[di][dj] = res_mat[i][j] + 1;
//             queue.push_back((di, dj));
//         }
//     }
// }
// res_mat


// find all 0, then bfs.
// use HashSet::new() to check if it has been visited


// Runtime88 ms
// Beats
// 5.13%
// Memory3.1 MB
// Beats
// 64.10%

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let sz1 = mat.len();
        let sz2 = mat[0].len();
        let mut vvi = vec![vec![123456789; sz2]; sz1];

        for i in 0..sz1 {
            for j in 0..sz2 {
                if mat[i][j] == 0 {
                    vvi[i][j] = 0;
                } else {
                    if i > 0 {
                        vvi[i][j] = vvi[i][j].min(vvi[i - 1][j] + 1);
                    }
                    if j > 0 {
                        vvi[i][j] = vvi[i][j].min(vvi[i][j - 1] + 1);
                    }
                }
            }
        }

        // for i in (sz1 - 1)..0 {
        //     for j in (sz2 - 1)..0 {

        // for mut i in 0..sz1 {
        //     for mut j in 0..sz2 {
        //         i = sz1 - 1 - i;
        //         j = sz2 - 1 - j;
        for i2 in 0..sz1 {
            for j2 in 0..sz2 {
                let i = sz1 - 1 - i2;
                let j = sz2 - 1 - j2;

                println!("{}, {}", i, j);

                if i + 1 < sz1 {
                    vvi[i][j] = vvi[i][j].min(vvi[i + 1][j] + 1);
                }
                if j + 1 < sz2 {
                    vvi[i][j] = vvi[i][j].min(vvi[i][j + 1] + 1);
                }
            }
        }
        vvi
    }
}




struct Solution {}

fn main()
{
    // let vvi = [[0,0,0].to_vec(),[0,1,0].to_vec(),[1,1,1].to_vec()].to_vec();
    let vvi = [[1,1,0,0,1,0,0,1,1,0].to_vec(),[1,0,0,1,0,1,1,1,1,1].to_vec(),[1,1,1,0,0,1,1,1,1,0].to_vec(),[0,1,1,1,0,1,1,1,1,1].to_vec(),[0,0,1,1,1,1,1,1,1,0].to_vec(),[1,1,1,1,1,1,0,1,1,1].to_vec(),[0,1,1,1,1,1,1,0,0,1].to_vec(),[1,1,1,1,1,0,0,1,1,1].to_vec(),[0,1,0,1,1,0,1,1,1,1].to_vec(),[1,1,1,0,1,0,1,1,1,1].to_vec()].to_vec();

    println!("ans: {:?}", Solution::update_matrix(vvi));
}


