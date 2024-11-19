


// D D

// int n = word.size(), res = n;
// for (int i = 1; i < n; ++i)
//     res -= word[i] != word[i - 1];




// Runtime
// 0ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.14MB
// Beats33.33%

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ans = 0;
        let s = word.as_bytes();
        let mut cnt = 0;
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                cnt += 1;
            } else {
                ans += cnt;
                cnt = 0;
            }
        }

        ans + cnt + 1
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


