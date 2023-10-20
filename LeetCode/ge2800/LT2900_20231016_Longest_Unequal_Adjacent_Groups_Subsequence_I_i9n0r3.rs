





// Runtime3 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory2.1 MB
// Beats
// 100%

impl Solution {
    pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut t2 = groups[0];
        // ans.push(words[0]); // move occurs because value has type `std::string::String`, which does not implement the `Copy` trait
        ans.push(words[0].clone());
        for i in 1..words.len() {
            if t2 != groups[i] {
                ans.push(words[i].clone());
                t2 = groups[i];
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


