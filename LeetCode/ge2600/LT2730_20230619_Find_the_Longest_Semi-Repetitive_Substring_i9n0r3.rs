




// Runtime8 ms
// Beats
// 82.76%
// Memory2 MB
// Beats
// 96.55%

// 2 2345543 3
// continue at most 1
impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }
        let mut ans = 2;
        let vc = s.as_bytes();
        let mut st = 0;     // +1
        let mut st2 = 0;        // +1
        for i in 1..vc.len() {
            if vc[i] == vc[i - 1] {
                if st2 == 0 {
                    ans = ans.max(i - st + 1);
                } else {
                    ans = ans.max(i - st);
                }
                st = st2;
                st2 = i;
            }
        }
        ans = ans.max(vc.len() - st);
        ans as i32
    }
}




struct Solution {}

fn main()
{

    let a = "223455433".to_string();

    println!("ans: {:?}", Solution::longest_semi_repetitive_substring(a));
}


