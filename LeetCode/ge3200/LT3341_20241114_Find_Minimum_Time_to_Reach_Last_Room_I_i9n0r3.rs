




// Runtime
// 6ms
// Beats49.12%
// Analyze Complexity
// Memory
// 2.22MB
// Beats28.07%


// move into
// start moving  ==  next second into 

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        
        use std::collections::BinaryHeap;

        let mut priq = BinaryHeap::new();

        priq.push((0i32, 0i32, 0i32));   // -second, i, j

        let mut vst = vec![vec![false; move_time[0].len()]; move_time.len()];
        vst[0][0] = true;
        let arr = [1i32,0,-1,0,1].to_vec();
        let (mut ni, mut nj, mut ntm) = (0,0,0);
        loop {
            match priq.pop() {
                Some((tm2, i, j)) => {

                    // println!("{}, {} {}", tm2, i, j);

                    if i as usize == move_time.len() - 1 && j as usize == move_time[0].len() - 1 {
                        return -tm2;
                    }
                    let tm = -tm2;
                    for m in 1..arr.len() {
                        ni = i + arr[m - 1];
                        nj = j + arr[m];
                        if ni >= 0 && nj >= 0 
                            && (ni as usize) < move_time.len() && (nj as usize) < move_time[0].len() 
                            && !vst[ni as usize][nj as usize] {

                            vst[ni as usize][nj as usize] = true;
                            ntm = (tm + 1).max(1 + move_time[ni as usize][nj as usize]);
                            priq.push((-ntm, ni, nj));
                        }
                    }
                },
                None => break,
            }
        }
        unreachable!()
    }
}


struct Solution {}

fn main()
{

    let vvi = [[0,4].to_vec(), [4,4].to_vec()].to_vec();

    println!("ans: {:?}", Solution::min_time_to_reach(vvi));
}


