

// D D


// let mut str_rows: Vec<(i32, usize)> = mat.iter()
// .enumerate()
// .map(|(idx, row)| (row.iter().sum(), idx))
// .collect();
// str_rows.sort();
// str_rows.iter()
// .take(k as usize)
// .map(|(_, idx)| *idx as i32)
// .collect()


// Runtime2 ms
// Beats
// 70%
// Memory2.2 MB
// Beats
// 80%


impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut k = k;
        for j in 0..(mat[0].len()) {
            if k == 0 {
                break;
            }
            for i in 0..mat.len() {
                if k == 0 {
                    break;
                }
                if mat[i][j] == 0 && (j == 0 || mat[i][j - 1] == 1) {
                    // println!("{}, {}, {}", i, j, mat[i][j]);
                    ans.push(i as i32);
                    k -= 1;
                }
            }
        }
        for i in 0..mat.len() {
            if k == 0 {
                break;
            }
            if mat[i][mat[0].len() - 1] == 1 {
                ans.push(i as i32);
                k -= 1;
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{
    // let vvi = [[1,0,0,0].to_vec(), [1,1,1,1].to_vec(), [1,0,0,0].to_vec(), [1,0,0,0].to_vec()].to_vec();
    // let k = 2;

    let vvi = [[1,1,1,1,1].to_vec(),[1,0,0,0,0].to_vec(),[1,1,0,0,0].to_vec(),[1,1,1,1,0].to_vec(),[1,1,1,1,1].to_vec()].to_vec();
    let k = 3;

    println!("ans: {:?}", Solution::k_weakest_rows(vvi, k));
}


