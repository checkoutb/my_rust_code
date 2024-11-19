





// II
// Runtime
// 20ms
// Beats96.67%
// Analyze Complexity
// Memory
// 4.82MB
// Beats88.33%



// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.36MB
// Beats86.05%



impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut vi = [0; 26];
        let mut cnt = 0;
        let mut ans = 0i64;

        let w2 = word2.as_bytes();
        let au8 = b'a';

        for ch in w2 {
            let t2 = (ch - au8) as usize;
            if vi[t2] == 0 {
                cnt += 1;
            }
            vi[t2] += 1;
        }

        let w1 = word1.as_bytes();
        let mut st = 0usize;
        for i in 0..w1.len() {
            let t2 = (w1[i] - au8) as usize;
            vi[t2] -= 1;
            if vi[t2] == 0 {
                cnt -= 1;
            }

            while cnt == 0 {
                ans += (w1.len() - i) as i64;
                let t3 = (w1[st] - au8) as usize;
                st += 1;
                if vi[t3] == 0 {
                    cnt += 1;
                }
                vi[t3] += 1;
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


