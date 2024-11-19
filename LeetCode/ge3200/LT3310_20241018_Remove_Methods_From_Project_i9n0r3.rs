


// D D








// Runtime
// 70ms
// Beats100.00%
// Analyze Complexity
// Memory
// 19.57MB
// Beats75.56%




// the fk * and as


// k 及 k调用的方法 是 suspicious的

// 组外的方法 不会调用 组内的， 这个组可以删除

// 这绕了半天，感觉就是 k 及 被k调用的 要形成 一个 不会被别人调用。

// 就是不能 被 其他正常的方法 调用

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {

        if n == 1 {    // ...
            return Vec::new();
        }

        use std::collections::VecDeque;

        let mut invoke = vec![Vec::new(); n as usize];
        let mut beInvoke = vec![Vec::new(); n as usize];

        for vi in invocations {
            invoke[vi[0] as usize].push(vi[1]);
            beInvoke[vi[1] as usize].push(vi[0]);
        }

        let mut flg = vec![0; n as usize];  // is suspicious ?

        let mut que = VecDeque::new();
        que.push_back(k);
        flg[k as usize] = 1;

        while !que.is_empty() {
            let t2 = que.pop_front().unwrap();
            for nxt in &invoke[t2 as usize] {
                if flg[*nxt as usize] == 0 {
                    flg[*nxt as usize] = 1;
                    que.push_back(*nxt);
                }
            }
        }

        let mut canRemove = true;

        for i in 0..flg.len() {
            if flg[i] == 1 {
                for t2 in &beInvoke[i] {
                    if flg[*t2 as usize] == 0 {
                        canRemove = false;
                    }
                }
            }
        }

        let mut ans = Vec::new();

        if canRemove {
            for i in 0..flg.len() {
                if flg[i] == 0 {
                    ans.push(i as i32);
                }
            }
        } else {
            for i in 0..n {
                ans.push(i as i32);
            }
        }
        ans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


