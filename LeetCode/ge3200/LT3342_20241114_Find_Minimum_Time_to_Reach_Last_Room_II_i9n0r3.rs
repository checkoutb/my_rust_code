




// Runtime
// 104ms
// Beats90.91%
// Analyze Complexity
// Memory
// 9.50MB
// Beats100.00%


// 1, 2


impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        
        use std::collections::BinaryHeap;

        // -tm, next_step, i, j
        let mut priq: BinaryHeap<(i32,i32,i32,i32)> = BinaryHeap::new();
        priq.push((0, 1, 0, 0));
        let mut ans = -1i32;
        let (mut ni, mut nj, mut ntm) = (0,0,0);
        let arr = [1,0,-1,0,1];
        let mut vvi = vec![vec![i32::MAX; move_time[0].len()]; move_time.len()];
        vvi[0][0] = 0;
        let mut ans = -1i32;

        loop {
            match priq.pop() {
                Some((tm2, nxt, i, j)) => {
                    // println!("{},{},{},{}", tm2, nxt, i, j);
                    let tm = -tm2;
                    if i as usize == move_time.len() - 1 && (j as usize) == move_time[0].len() - 1 {
                        ans = tm;
                        break;
                    }
                    for m in 1..arr.len() {
                        (ni, nj) = (i + arr[m - 1], j + arr[m]);

                        if ni >= 0 && nj >= 0 && (ni as usize) < move_time.len() 
                            && (nj as usize) < move_time[0].len() {
                            ntm = nxt + tm.max(move_time[ni as usize][nj as usize]);
                            if ntm < vvi[ni as usize][nj as usize] {
                                vvi[ni as usize][nj as usize] = ntm;
                                priq.push((-ntm, 3 - nxt, ni, nj));
                            }
                        }                        
                    }
                },
                None => break,
            }
        }

        ans
    }
}



struct Solution {}

fn main()
{

    let vvi = [[0,4].to_vec(), [4,4].to_vec()].to_vec();

    println!("ans: {:?}", Solution::min_time_to_reach(vvi));
}


