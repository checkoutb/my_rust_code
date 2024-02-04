
// D D


// let mut v: Vec<_> = end_time
//     .into_iter()
//     .zip(start_time)
//     .zip(profit)
//     .map(|((x, y), z)| (x, y, z))
//     .collect();
// v.sort_unstable();
// let mut dp = Vec::with_capacity(v.len() + 1);
// dp.push(0);
// for (i, &(_, s, p)) in v.iter().enumerate() {
//     let j = v[..i].partition_point(|&(e, _, _)| e <= s);
//     dp.push(dp[i].max(dp[j] + p));
// }
// *dp.last().unwrap()



// pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
//     let n = start_time.len();
//     let mut indices = (0..n).collect::<Vec<_>>();
//     indices.sort_unstable_by(|&i, &j| start_time[i].cmp(&start_time[j]));
//     let mut max_profit = vec![0; n + 1];
//     for i in (0..n).rev() {
//         let end = end_time[indices[i]];
//         let next_i = indices[i + 1..]
//             .binary_search_by(|&index| match start_time[index].cmp(&end) {
//                 Ordering::Less => Ordering::Less,
//                 Ordering::Equal | Ordering::Greater => Ordering::Greater,
//             })
//             .err()
//             .unwrap()
//             + i
//             + 1;
//         max_profit[i] = max_profit[i + 1].max(profit[indices[i]] + max_profit[next_i]);
//     }
//     max_profit[0]
// }



// Runtime17 ms
// Beats
// 55.56%
// Memory4 MB
// Beats
// 55.56%


// st,en,pf
// sort by st
// sort by en
// en -> profit
// pri-que
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut priq: BinaryHeap<(i32, i32)> = BinaryHeap::new();       // <end, all profit>
        let mut vt: Vec<(i32,i32,i32)> = Vec::new();
        let sz1 = start_time.len();
        for i in 0..sz1 {
            vt.push((start_time[i], end_time[i], profit[i]));
        }
        vt.sort();
        let mut pmp = 0i32;     // prefix mx profit
        for i in 0..sz1 {
            let (st, en, pf) = vt[i];
            // while !priq.is_empty() && priq.f
            while priq.peek().unwrap_or(&(i32::MIN, 0)).0 >= -st {
                pmp = pmp.max(priq.pop().unwrap().1);
            }
            priq.push((-en, pf + pmp));     // max-heap, so -en
        }
        // while !priq.is_empty() {
        // }
        for x in priq.iter() {
            pmp = pmp.max(x.1);
        }
        pmp
    }
}




struct Solution {}

fn main()
{
    let vs = [1,2,3,4,6].to_vec();
    let ve = [3,5,10,6,9].to_vec();
    let pf = [20,20,100,70,60].to_vec();

    println!("ans: {:?}", Solution::job_scheduling(vs,ve,pf));
}


