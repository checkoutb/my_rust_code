


// use std::collections::BinaryHeap;
// use std::cmp::Reverse;

// impl Solution {
//     pub fn full_bloom_flowers(mut flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
//         flowers.sort_by(|a, b| b[0].cmp(&a[0]));
//         let mut sorted_people: Vec<(i32, usize)> = people.into_iter().enumerate().map(|(i, t)| (t, i)).collect();
//         sorted_people.sort_unstable();

//         let mut in_bloom: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
//         let mut ans = vec![0; sorted_people.len()];

//         for (t, i) in sorted_people {
//             while ! in_bloom.is_empty() && in_bloom.peek().unwrap().0 < t {
//                 in_bloom.pop();
//             }

//             while ! flowers.is_empty() && flowers.last().unwrap()[0] <= t {
//                 let f = flowers.pop().unwrap();
//                 if f[1] >= t {
//                     in_bloom.push(Reverse(f[1]));
//                 }
//             }

//             ans[i] = in_bloom.len() as i32;
//         }

//         ans
//     }
// }



// let mut people = people
// .into_iter()
// .enumerate()
// .map(|(i, p)| (p, i))
// .collect::<Vec<_>>();


// Runtime45 ms
// Beats
// 63.64%
// Memory8.5 MB
// Beats
// 9.9%

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut vst: Vec<i32> = Vec::new();
        let mut ven: Vec<i32> = Vec::new();
        for vi in flowers.iter() {
            vst.push(vi[0]);
            ven.push(vi[1]);
        }
        vst.sort();
        ven.sort();

        let mut vp: Vec<Vec<i32>> = Vec::new();
        
        for (i, pos) in people.into_iter().enumerate() {
            vp.push([pos, i as i32, 0].to_vec());
        }
        vp.sort();

        let mut sti: usize = 0;
        let mut eni: usize = 0;
        let mut cnt: i32 = 0;
        for i in 0..vp.len() {
            let pos = vp[i][0];
            while (sti < vst.len()) && (vst[sti] <= pos) {
                cnt += 1;
                sti += 1;
            }
            while (eni < ven.len()) && (ven[eni] < pos) {
                cnt -= 1;
                eni += 1;
            }
            vp[i][2] = cnt;
        }
        let mut ans: Vec<i32> = vec![0; vp.len()];
        for vi in vp.iter() {
            ans[vi[1] as usize] = vi[2];
        }
        ans
    }
}






struct Solution {}

fn main()
{

    let vvi = [[1,6].to_vec(), [3,7].to_vec(), [9,12].to_vec(), [4,13].to_vec()].to_vec();
    let p = [2,3,7,11].to_vec();


    println!("ans: {:?}", Solution::full_bloom_flowers(vvi, p));
}


