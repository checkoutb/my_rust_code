

// D D

// if image[i-2..i+1].iter().any(|row| row[j-2].abs_diff(row[j-1])>t || row[j-1].abs_diff(row[j])>t){
//     continue
// }
// if (j-2..j+1).any(|y| image[i-2][y].abs_diff(image[i-1][y])>t || image[i-1][y].abs_diff(image[i][y])>t){
//     continue
// }
// let avg=image[i-2..i+1].iter().map(|row| row[j-2..j+1].iter().sum::<i32>()).sum::<i32>()/9;





// Runtime
// 146ms
// Beats66.67%of users with Rust
// Memory
// 5.29MB
// Beats81.48%of users with Rust

impl Solution {
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let sz1 = image.len();
        let sz2 = image[0].len();
        let mut vvi = vec![vec![-1; sz2]; sz1];
        // let mut region = vec![vec![true; sz2]; sz1];

        // Self::cal_region(&image, &region, threshold);

        for i in 1..(sz1 - 1) {
            let mut sum2 = image[i-1][0] + image[i][0] + image[i+1][0] + image[i-1][1] + image[i][1] + image[i+1][1];
            for j in 1..(sz2 - 1) {
                sum2 += image[i-1][j+1] + image[i][j+1] + image[i+1][j+1];
                if Self::is_region(&image, i, j, threshold) {
                    vvi[i][j] = sum2 / 9;
                }
                sum2 -= image[i-1][j-1] + image[i][j-1] + image[i+1][j-1];
            }
        }

        let mut vv2 = vec![vec![0; sz2]; sz1];

        // println!("{:?}", vvi);

        for i in 0..sz1 {
            for j in 0..sz2 {
                vv2[i][j] = Self::cal_average(&vvi, i as i32, j as i32, image[i][j]);
            }
        }

        vv2
    }

    fn is_region(image: &Vec<Vec<i32>>, i: usize, j: usize, mx: i32) -> bool {
        let vi = [1,0,1].to_vec();
        for a in 0..=1 {
            for b in 0..=1 {
                for m in 1..vi.len() {
                    let (ni, nj) = (i - a + vi[m - 1], j - b + vi[m]);
                    if Self::abs(image[i - a][j - b] - image[ni][nj]) > mx {
                        return false;
                    }
                }
            }
        }

        for a in 0..=1 {
            let (ni, nj) = (i + 1, j - 1 + a);
            if Self::abs(image[ni][nj] - image[ni][nj + 1]) > mx {
                return false;
            }
            let (mi, mj) = (i - 1 + a, j + 1);
            if Self::abs(image[mi][mj] - image[mi + 1][mj]) > mx {
                return false;
            }
        }

        true
    }

    // fn cal_region(image: &Vec<Vec<i32>>, region: &Vec<Vec<bool>>, mx: i32) {
    //     for i in 0..image.len() {
    //         for j in 0..image[0].len() {
    //             Self::fill_region(image, region, i, j, mx);
    //         }
    //     }
    // }

    // fn fill_region(image: &Vec<Vec<i32>>, region: &Vec<Vec<bool>>, i: usize, j: usize, mx: i32) {
    //     // if !region[i][j] {
    //     //     return;
    //     // }
    //     // let vi = [1,0,-1,0,1].to_vec();
    //     let vi = [1,0,1].to_vec();
    //     for k in 1..vi.len() {
    //         let (ni, nj) = (i as i32 + vi[k - 1], j as i32 + vi[k]);
    //         if ni < 0 || nj < 0 || ni == image.len() || nj == image[0].len() {
    //             continue;
    //         }
    //         if Self::abs(image[i][j] - image[ni][nj]) > mx {
    //             region[i][j] = false;
    //             region[ni][nj] = false;
    //             break;
    //         }
    //     }
    // }

    fn abs(mut n: i32) -> i32 {
        if n < 0 {
            n = -n;
        }
        n
    }

    fn cal_average(vvi: &Vec<Vec<i32>>, i: i32, j: i32, img: i32) -> i32 {
        let mut sum2 = 0;
        let mut cnt = 0;

        // println!("{}, {}", i, j);

        for a in -1..=1 {
            for b in -1..=1 {
                let (ni, nj) = (i + a, j + b);

                // print!("{}, {} --- ", ni, nj);
                // print!("{},{},  ,",a,b);

                if ni < 0 || nj < 0 || ni == vvi.len() as i32 || nj == vvi[0].len() as i32 {
                    continue;
                }
                if vvi[ni as usize][nj as usize] != -1 {
                    sum2 += vvi[ni as usize][nj as usize];
                    cnt += 1;
                }
            }
        }
        // let vi = [1,0,-1,0,1].to_vec();
        // for k in 1..vi.len() {
        //     let (ni, nj) = (i + vi[k - 1], j + vi[k]);
        //     if ni < 0 || nj < 0 || ni == vvi.len() as i32 || nj == vvi[0].len() as i32 {
        //         continue;
        //     }
        //     if vvi[ni as usize][nj as usize] != -1 {
        //         sum2 += vvi[ni][nj];
        //         cnt += 1;
        //     }
        // }

        // println!("----------- {}, {}", sum2, cnt);

        if cnt == 0 {
            img
        } else {
            sum2 / cnt
        }
    }

    // (value, count)
    // fn get_val(vvi: &Vec<Vec<i32>>, i i32, j i32) -> (i32, i32) {

    //     if i < 0 || j < 0 || i == vvi.len() as i32 || j == vvi[0].len() as i32 {
    //         return (0i32, 0i32);
    //     }

    //     if vvi[i as usize][j as usize] == -1 {
    //         (0i32, 0i32)
    //     } else {
    //         (vvi[i as usize][j as usize], 1i32)
    //     }
    // }
}



struct Solution {}

fn main()
{

    // let vvi = [[5,6,7,10].to_vec(),[8,9,10,10].to_vec(),[11,12,13,10].to_vec()].to_vec();
    // let k = 3;

    let vvi = [[4,8,5].to_vec(),[5,7,7].to_vec(),[9,4,2].to_vec()].to_vec();
    let k = 4;

    println!("ans: {:?}", Solution::result_grid(vvi, k));


    // for a in -2..2 {             // ..2 ..=2
    //     print!("{},", a);
    // }

}


