




// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.34MB
// Beats20.00%


// point.len < 11

impl Solution {
    pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort();
        let mut ans = -1;
        for i in 1..points.len() {
            if points[i][0] == points[i - 1][0] {
                let mut y0mnx = i32::MAX;
                let mut y1mnx = i32::MAX - 1;
                // let mut mnx = i32::MAX - 2;
                for j in (i + 1)..points.len() {
                    if points[j][1] == points[i - 1][1] {
                        y0mnx = y0mnx.min(points[j][0]);
                    }
                    if points[j][1] == points[i][1] {
                        y1mnx = y1mnx.min(points[j][0]);
                    }
                    if points[j][1] > points[i - 1][1] && points[j][1] < points[i][1] {
                        break;
                    }
                }
                if y0mnx == y1mnx {
                    ans = ans.max((y0mnx - points[i][0]) * (points[i][1] - points[i - 1][1]));
                }
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


