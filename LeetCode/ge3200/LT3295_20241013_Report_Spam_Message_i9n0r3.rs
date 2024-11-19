

// D D


// let banned_words: std::collections::HashSet<_> = banned_words.into_iter().collect();
// let has = |m| banned_words.contains(m);
// let mut it = message.iter();
// it.by_ref().any(has);
// it.by_ref().any(has)


// let mut s: HashSet<String> = HashSet::from_iter(banned_words.into_iter());
// message.into_iter().filter(|msg| s.contains(msg)).take(2).count() == 2





// Runtime
// 326ms
// Beats5.97%
// Analyze Complexity
// Memory
// 16.84MB
// Beats83.58%


impl Solution {
    pub fn report_spam(mut message: Vec<String>, mut banned_words: Vec<String>) -> bool {

        use std::cmp::Ordering;

        message.sort();
        banned_words.sort();

        let mut midx = 0;
        let mut bidx = 0;
        let mut cnt = 0;
        while midx < message.len() && bidx < banned_words.len() && cnt < 2 {
            match message[midx].cmp(&banned_words[bidx]) {
                Ordering::Less => midx += 1,
                Ordering::Equal => {
                    midx += 1;
                    cnt += 1;
                },
                Ordering::Greater => bidx += 1,
            }
        }

        cnt >= 2
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


