






// Runtime
// 75ms
// Beats66.67%
// Memory
// 11.75MB
// Beats66.67%


impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        
        let mut rectangles = rectangles;
        rectangles.sort();

        // println!("{:?}", rectangles);

        let mut cnt = 0;
        let mut en = rectangles[0][2];
        for i in 0..rectangles.len() {
            if rectangles[i][0] >= en {
                cnt += 1;
                if cnt == 2 {
                    return true;
                }
            }
            en = en.max(rectangles[i][2]);
        }

        rectangles.sort_by(|a, b| 
            if a[1] == b[1] {
                a[3].cmp(&b[3])
            } else {
                a[1].cmp(&b[1])
            }
        );
        // println!("{:?}", rectangles);
        cnt = 0;
        en = rectangles[0][3];
        for i in 0..rectangles.len() {
            if rectangles[i][1] >= en {
                cnt += 1;
                if cnt == 2 {
                    return true;
                }
            }
            en = en.max(rectangles[i][3]);
        }

        false
    }
}


struct Solution {}

fn main()
{

    let vvi = [[0,2,2,4].to_vec(),[1,0,3,2].to_vec(),[2,2,3,4].to_vec(),
    [3,0,4,2].to_vec(),[3,2,4,4].to_vec()].to_vec();

    println!("ans: {:?}", Solution::check_valid_cuts(0, vvi));
}


