

// D D

// for q in queries {
//     pq.push(q[0].abs() + q[1].abs());
//     if pq.len() > k as usize {
//         pq.pop();
//     }
//     ans.push(if pq.len() < k as usize { -1 } else { *pq.peek().unwrap() });
// }



// Runtime
// 106ms
// Beats46.43%
// Analyze Complexity
// Memory
// 17.34MB
// Beats66.07%



impl Solution {


    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        // use std::cmp::Reverse;

        // 写反了，这个应该是 mxheap。。
        let mut mnheap: BinaryHeap<i32> = BinaryHeap::new();
        let mut ans = vec![-1; queries.len()];

        for i in 0..queries.len() {
            let t2 = queries[i][0].abs() + queries[i][1].abs();
            // mnheap.push(Reverse(t2));
            mnheap.push(t2);
            while mnheap.len() > k as usize {
                mnheap.pop();
            }

            if mnheap.len() == k as usize {
                ans[i] = *mnheap.peek().unwrap();
            }
        }

        ans
    }
}

struct Solution {}

fn main()
{


//    println!("ans: {:?}", Solution::());
}


