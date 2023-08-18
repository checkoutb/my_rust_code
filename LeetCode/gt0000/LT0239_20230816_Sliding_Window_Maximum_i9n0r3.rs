


// D D

// let mut queue = VecDeque::new();
// let mut output = vec![];
// let mut first_val = i32::MIN;
// for i in 0..k as usize {
//     while !queue.is_empty() && nums[queue[queue.len() - 1]] < nums[i] {
//         queue.pop_back();
//     }
//     queue.push_back(i);
// }
// output.push(nums[queue[0]]);
// for i in k as usize..nums.len() {
//     //println!("{queue:?} {i} {k}");
//     if !queue.is_empty() && queue[0] == i - k as usize {
//         queue.pop_front();
//     }
//     while !queue.is_empty() && nums[queue[queue.len() - 1]] < nums[i] {
//         queue.pop_back();
//     }
//     queue.push_back(i);
//     output.push(nums[queue[0]]);
// }
// output
//
// 单调。




// Runtime64 ms
// Beats
// 69.29%
// Memory4.8 MB
// Beats
// 5.51%

impl Solution {
    // priority_queue<<value, index>> ....
    // .... BinaryHeap
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // use priority_queue::PriorityQueue;
        // let mut priq = PriorityQueue::new();

        use std::collections::BinaryHeap;
        let mut hp = BinaryHeap::new();

        for i in 0..(k as usize - 1) {
            hp.push((nums[i], i));
        }
        let mut vi = vec![];

        for i in (k as usize - 1)..nums.len() {
            hp.push((nums[i], i));
            while hp.len() > 0 {
                if let Some((_val, idx)) = hp.peek() {
                    if idx + k as usize <= (i) {
                        hp.pop();
                    } else {
                        break;
                    }
                }
            }
            if let Some((val, _idx)) = hp.peek() {
                vi.push(*val);
            }
        }

        vi
    }
}




struct Solution {}

fn main()
{

    let vi = [1,3,-1,-3,5,3,6,7].to_vec();
    let k = 3;

    // let vi = [1,-1].to_vec();
    // let k = 1;

    println!("ans: {:?}", Solution::max_sliding_window(vi, k));
}


