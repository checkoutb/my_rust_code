


// D D

// 直接暴力 截取 长度是 mxlen 的substr，然后 里面 最大的就是 ans。。。。
// 不是，`cur = word.substring(i, Math.min(i + m, n));`  尾巴上的substr 的长度小于 mxlen



// Runtime
// 38ms
// Beats18.75%
// Memory
// 2.39MB
// Beats59.38%




// 不可再分，或者 分法 无法再 distinct。
// 最大的字典序，  不太理解 和 round 有什么关系？ 我把 最大的 字典序 的 string 放到 最后一次round中 split 不就可以了吗？
// 看懂了，是 所有的拆分都尝试后， 要你找出 出现过的 最大的 字典序，不是 最后一次round的结果。

// 所以只是找到最大字典序，且 能 num_friends 个？
// 但是数据结构不好弄，没什么合适的。 碰到 qqqqqqqqqqq 直接爆炸

// 所以如果 前面和 自己一样，那么自己就没有必要 进入候选。

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {

        if num_friends == 1 {
            return word;
        }

        let s = word.into_bytes();
        let mut vi = vec![];
        let mut mx = b'a';
        for i in 0..s.len() {
            if mx < s[i] {
                mx = s[i];
            }
        }
        let mxlen = s.len() - num_friends as usize + 1;  // ans's max length
        for i in 0..s.len() {
            if s[i] == mx {
                if i > 0 && s[i] == s[i - 1] {
                    continue;
                }

                vi.push((i, i));  // substring's [start, end]
            }
        }
        let mut ans = (2, 0);
        while !vi.is_empty() {  // should check is mxlen...but..  b'1'
            let mut v2 = vec![];

            mx = b'1';
            for i in 0..vi.len() {
                let t2 = vi[i].1 + 1;
                if t2 < s.len() && mx < s[t2] {
                    mx = s[t2];
                }

            }
            // println!("{} {}", b'1', mx);
            if mx == b'1' || (vi[0].1 - vi[0].0 + 1) == mxlen {  // check
                // ans = vi[0];   // 
            } else {
                for i in 0..vi.len() {
                    let t2 = vi[i].1 + 1;
                    if t2 < s.len() && s[t2] == mx {
                        v2.push((vi[i].0, t2));
                    }
                }
            }
            if v2.is_empty() {
                ans = vi[0];     // vi.len()  maybe ==1, or all substr are mxlen
            }
            vi = v2;
        }
        String::from_utf8_lossy(&s[ans.0 ..= ans.1]).to_string()
    }
}


struct Solution {}

fn main()
{

    let s = "gh".to_string();
    let n = 1;

    println!("ans: {:?}", Solution::answer_string(s, n));
}


