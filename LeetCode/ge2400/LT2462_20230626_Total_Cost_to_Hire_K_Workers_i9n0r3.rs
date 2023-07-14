



// leetcode 崩了。。今天断断续续



// Runtime31 ms
// Beats
// 58.62%
// Memory3.6 MB
// Beats
// 51.72%

impl Solution {
    // use std::collections::BinaryHeap;        // traits, impl not support use xxx
    // use std::cmp::Reverse;

    // const BIT: i64 = 17;    // 1 << 17
    const MOD: i64 = 100000;

    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        // let sz1 = costs.len();
        let mut li: usize = 0;
        let mut ri: usize = costs.len() - 1;
        let mut priq = BinaryHeap::new();

        // for ((li < (candidates)) && (li <= ri)) {
        while (li < (candidates as usize)) && (li <= ri) {
            // priq.push(Reverse())
            // priq.push((((costs[li] as i64) * MOD) + ((sz1 - li) as i64)).wrapping_neg());
            priq.push(Reverse(costs[li] as i64 * Self::MOD + li as i64));
            if li != ri {
                priq.push(Reverse(costs[ri] as i64 * Self::MOD + ri as i64));
                // priq.push((((costs[ri] as i64) * MOD) + ((sz1 - ri) as i64)).wrapping_neg());
            }
            li += 1;
            if ri > 0 {
                ri -= 1;
            }
        }
        let mut k = k;
        let mut ans: i64 = 0;
        while k > 0 {
            k -= 1;

            match priq.pop() {
                Some(val) => {
                    let val = val.0;
                    ans += val / Self::MOD;
                    if li <= ri {
                        if val % Self::MOD < ri as i64 {
                            priq.push(Reverse(costs[li] as i64 * Self::MOD + li as i64));
                            li += 1;
                        } else {
                            priq.push(Reverse(costs[ri] as i64 * Self::MOD + ri as i64));
                            if ri > 0 {
                                ri -= 1;
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
    let vi: Vec<i32> = [17,12,10,2,7,2,11,20,8].to_vec();
    let k: i32 = 3;
    let candidates = 4;

    println!("ans: {:?}", Solution::total_cost(vi, k, candidates));
}


