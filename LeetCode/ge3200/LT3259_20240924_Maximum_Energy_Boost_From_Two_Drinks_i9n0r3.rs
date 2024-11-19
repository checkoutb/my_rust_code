

// let (mut max_a, mut max_b, mut max_none) = (0, 0, 0);
// for i in 0..n {
//     (max_a, max_b, max_none) = (
//         max_a.max(max_none) + energy_drink_a[i] as i64,
//         max_b.max(max_none) + energy_drink_b[i] as i64,
//         max_a.max(max_b)
//     )
// }
// max_a.max(max_b)

// 同时发生，所以 max_none 的值 是 max_a max_b 还没有执行的时候。


// Runtime
// 49ms
// Beats28.30%
// Analyze Complexity
// Memory
// 8.48MB
// Beats39.62%

// 切换 需要消耗 1小时

impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let sz1 = energy_drink_a.len();
        let mut vvi = vec![vec![0i64; 2]; sz1];
        
        vvi[0][0] = energy_drink_a[0] as i64;
        vvi[0][1] = energy_drink_b[0] as i64;
        vvi[1][0] = vvi[0][0] + energy_drink_a[1] as i64;
        vvi[1][1] = vvi[0][1] + energy_drink_b[1] as i64;

        for idx in 2..sz1 {
            vvi[idx][0] = energy_drink_a[idx] as i64 + vvi[idx - 1][0].max(vvi[idx - 2][1]);
            vvi[idx][1] = energy_drink_b[idx] as i64 + vvi[idx - 1][1].max(vvi[idx - 2][0]);
        }

        // println!("{:?}", vvi);

        vvi[sz1 - 1][0].max(vvi[sz1 - 1][1])
    }
}


struct Solution {}

fn main()
{
    let vi = [3,3,3].to_vec();
    let v2 = [3,4,6].to_vec();   // 13

    println!("ans: {:?}", Solution::max_energy_boost(vi, v2));
}


