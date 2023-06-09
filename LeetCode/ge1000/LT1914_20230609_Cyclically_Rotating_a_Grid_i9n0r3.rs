






// Runtime4 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.2 MB
// Beats
// 100%


// struct Point {
//     i: usize,
//     j: usize,
//     dir: usize,
// }

struct Point(usize, usize, usize);

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let sz1 = grid.len();
        let sz2 = grid[0].len();
        let mut grid = grid;
        for i in 0..(sz1 / 2).min(sz2 / 2) {
            let sz3 = (sz1 - i * 2) * 2 + (sz2 - i * 2) * 2 - 4;
            let kk = (k as usize) % (sz3);
            let mut vi = vec![0; sz3];
            let mut pt = Point(i, i, 0);
            let mxi = sz1 - i - 1;
            let mni = i;
            let mxj = sz2 - i - 1;
            let mnj = i;
            for j in 0..sz3 {
                let Point(x, y, _) = pt;
                vi[j] = grid[x][y];
                pt = Self::nxt_pt(pt, mxi, mxj, mni, mnj);
            }
            
            // println!("{:?}", vi);

            pt = Point(i, i, 0);
            for j in 0..sz3 {
                let Point(x, y, _) = pt;
                grid[x][y] = vi[(sz3 + j - kk) % sz3];              // usize - usize + uszie will overflow.

                // println!("{}+{} = {}", x, y, ((sz3 + j - kk) % sz3));

                pt = Self::nxt_pt(pt, mxi, mxj, mni, mnj);
            }
        }
        grid
    }

    // return (i * 100 + j) * 10 + direct,  0down,1right,2up,3left
    fn nxt_pt(point: Point, mxi: usize, mxj: usize, mni: usize, mnj: usize) -> Point {
        // let (mut i, mut j, mut dir) = point;
        let Point(mut i, mut j, mut dir) = point;
        if dir == 0 {
            i += 1;
            if i > mxi {
                i -= 1;
                j += 1;
                dir = 1;
            }
        } else if dir == 1 {
            j += 1;
            if j > mxj {
                i -= 1;
                j -= 1;
                dir = 2;
            }
        } else if dir == 2 {
            if i > mni {
                i -= 1;
            } else {
                j -= 1;
                dir = 3;
            }
        } else {
            if j > mnj {
                j -= 1;
            } else {
                i += 1;
                dir = 0;
            }
        }
        Point(i, j, dir)
    }
}




struct Solution {}

fn main()
{

    // let vvi = [[1,2,3,4].to_vec(),[5,6,7,8].to_vec(),[9,10,11,12].to_vec(),[13,14,15,16].to_vec()].to_vec();
    // let k = 2;
    
    let vvi = [[4,5,8,9,4,2,4,7,2,4].to_vec(),[7,1,9,6,6,1,4,5,7,7].to_vec(),[7,1,5,1,1,7,10,1,3,1].to_vec(),[7,2,2,5,2,6,6,4,7,7].to_vec(),[1,2,3,8,4,7,6,9,6,2].to_vec(),[5,10,3,4,7,2,7,5,3,10].to_vec()].to_vec();
    let k = 4;

    for vi in &vvi {
        println!("{:?}", *vi);
    }

    println!("ans: {:?}", 111);

    let vvi = Solution::rotate_grid(vvi, k);
    for vi in vvi {
        println!("{:?}", vi);
    }
}



// [4, 5, 8, 9, 4, 2, 4, 7, 2, 4]
// [7, 1, 9, 6, 6, 1, 4, 5, 7, 7]
// [7, 1, 5, 1, 1, 7, 10, 1, 3, 1]
// [7, 2, 2, 5, 2, 6, 6, 4, 7, 7]
// [1, 2, 3, 8, 4, 7, 6, 9, 6, 2]
// [5, 10, 3, 4, 7, 2, 7, 5, 3, 10]
