

// D D


// mxpq not need, bsum is enough



// Runtime
// 13ms
// Beats100.00%of users with Rust
// Memory
// 3.05MB
// Beats66.67%of users with Rust

// max = ladder
// what ds?      2 pri_q   or  BinarySearch

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        use std::collections::BinaryHeap;       // max heap
        let mut mxpq = BinaryHeap::new();   // brick, smaller
        let mut mnpq = BinaryHeap::new();   // ladder, bigger, negative
        let mut bsum = 0i32;
        let mut ans = 0;
        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];
            if diff <= 0 {
                ans = i as i32;
                continue;
            }
            if mnpq.len() < ladders as usize {
                mnpq.push(-diff);
            } else {
                let t2 = -mnpq.peek().unwrap_or(&(i32::MIN + 1));       // ladders == 0
                if diff < t2 || ladders == 0 {
                    bsum += diff;
                    mxpq.push(diff);
                } else {
                    // swap
                    mnpq.pop(); // t2
                    mnpq.push(-diff);

                    bsum += t2;
                    mxpq.push(t2);
                }
            }
            
            if bsum > bricks {
                break;
            } else {
                ans = i as i32;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    // let vi = [4,2,7,6,9,14,12].to_vec();
    // let b = 5;
    // let l = 1;

    let vi = [14,3,19,3].to_vec();
    let b = 17;
    let l = 0;


    println!("ans: {:?}", Solution::furthest_building(vi, b, l));
}


