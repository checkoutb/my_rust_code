






// Runtime3 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2 MB
// Beats
// 100%

impl Solution {
    pub fn min_length(s: String) -> i32 {
        //let mut ans: i32 = 0;

        let mut vec = Vec::new();

        //for i in 0..s.len() {
        for ch in s.chars() {
            if vec.is_empty() {
                vec.push(ch);
            } else {
                if (ch == 'B' && vec.last().unwrap() == &'A') || (ch == 'D' && vec.last().unwrap() == &'C') {
                    vec.pop();
                } else {
                    vec.push(ch);
                }
            }
        }

        vec.len() as i32

        // ans = vec.len() as i32;
        // return ans;
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::min_length("ABFCACDB".to_string()));
}


