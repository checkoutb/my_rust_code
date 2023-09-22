
// D D

// impl Solution {
//     pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
//         let points = points.into_iter().map(|p| (p[0], p[1])).collect::<Vec<_>>();

//         let mut connected = vec![false; points.len()];
//         let mut dist = vec![i32::MAX; points.len()];

//         let mut min_idx = 0;
//         let mut min_cost = 0;

//         let mut total_cost = 0;

//         for _ in 0..points.len() {
//             connected[min_idx] = true;
//             total_cost += min_cost;

//             let cur_point = points[min_idx];
//             min_cost = i32::MAX;

//             for (idx, point) in points.iter().enumerate() {
//                 if !connected[idx] {
//                     dist[idx] = dist[idx]
//                         .min((cur_point.0 - point.0).abs() + (cur_point.1 - point.1).abs());
//                     if dist[idx] < min_cost {
//                         min_cost = dist[idx];
//                         min_idx = idx;
//                     }
//                 }
//             }
//         }
//         total_cost
//     }
// }





// Runtime253 ms
// Beats
// 10%
// Memory32.8 MB
// Beats
// 20%

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut vi: Vec<Vec<i32>> = Vec::new();
        for i in 0..points.len() {
            // let (a, b) = points[i][0], points[i][1];
            let a = points[i][0];
            let b = points[i][1];
            for j in (i + 1)..points.len() {
                vi.push([(a - points[j][0]).abs() + (b - points[j][1]).abs(), i as i32, j as i32].to_vec());
            }
        }
        vi.sort();

        println!("{:?}", vi);

        let mut uf: Vec<i32> = vec![-1; points.len()];
        let mut cnt: usize = 1;
        let mut ans = 0;
        for i in 0..vi.len() {
            if cnt == points.len() {
                break;
            }
            let a = Self::ufa(&mut uf, vi[i][1] as usize);
            let b = Self::ufa(&mut uf, vi[i][2] as usize);
            if a != b {
                uf[a as usize] = b;
                ans += vi[i][0];
                cnt += 1;
            }
        }
        ans
    }

    fn ufa(uf: &mut Vec<i32>, idx: usize) -> i32 {
        if uf[idx] == -1 {
            return idx as i32;
        }
        uf[idx] = Self::ufa(uf, uf[idx] as usize);
        return uf[idx];
    }
}



struct Solution {}

fn main()
{
    let vvi = [[0,0].to_vec(),[2,2].to_vec(),[3,10].to_vec(),[5,2].to_vec(),[7,0].to_vec()].to_vec();

    println!("ans: {:?}", Solution::min_cost_connect_points(vvi));
}


