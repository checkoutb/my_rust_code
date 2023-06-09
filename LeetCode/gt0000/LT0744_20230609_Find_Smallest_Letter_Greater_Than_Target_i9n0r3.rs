


// D D

// let i = letters.partition_point(|&c| c <= target);
// if i == letters.len() {letters[0]} else {letters[i]}



// Runtime6 ms
// Beats
// 58.33%
// Memory2.7 MB
// Beats
// 91.67%

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        // let fst: char = letters[0];
        for ch in &letters {
            if *ch > target {
                return *ch;
            }
        }
        return letters[0];
        // return *fst;
    }
}



struct Solution {}

fn main()
{

    let vc: Vec<char> = ['c','f','j'].to_vec();
    let tar: char = 'c';

    println!("ans: {:?}", Solution::next_greatest_letter(vc, tar));
}


