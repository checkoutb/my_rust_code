



// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.33MB
// Beats60.00%



// [[7,1],[19,3],[9,4],[12,5],[2,8],[15,10],[18,12],[7,14],[19,16]]  -> 2
// ? 看题目 应该是 和前面的相减 就是 自己的 消耗的时间啊， 19不是2+2么， 2只有3.
// 
// ...... 有人也问了， 有人回答说 是 单次push最长时间，不是 sum。  nm。


impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut btn = events[0][0];
        let mut mxtm = events[0][1];

        for i in 1..events.len() {
            let t2 = events[i][1] - events[i - 1][1];
            if t2 > mxtm {
                mxtm = t2;
                btn = events[i][0];
            } else if t2 == mxtm {
                if events[i][0] < btn {
                    btn = events[i][0];
                }
            }
        }
        btn
    }
}


impl Solution {
    pub fn button_with_longest_time__EE(events: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut map2: HashMap<i32, i32> = HashMap::new();
        map2.insert(events[0][0], events[0][1]);
        for i in 1..events.len() {
            let t2 = events[i][1] - events[i - 1][1];
            map2.entry(events[i][0]).and_modify(|v| *v += t2).or_insert(t2);
        }
        let mut btn = -1;
        let mut mxtm = -1;
        for (k, v) in map2.iter() {
            if *v > mxtm {
                mxtm = *v;
                btn = *k;
            } else if *v == mxtm {
                if *k < btn {
                    btn = *k;
                }
            }
        }
        btn
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


