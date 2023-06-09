
// D D

// let mut start = 0;
// for i in 1..=s.len()/2 {
//     if s.len()%i==0 {
//         if s[0..i].repeat(s.len()/i) == s {return true;}
//     }
// }
// false
//
// 6


// pub fn repeated_substring_pattern(mut s: String) -> bool {
//     (1..s.len()).filter(|size| s.len() % size == 0)
//         .any(|size| 
//             s.as_bytes().chunks(size).collect::<Vec<_>>().windows(2).all(|sub| sub[0] == sub[1])
//         )
// }
//
// 9


// s = s.repeat(2).to_owned();
// s[1..s.len() - 1].find(&s[..s.len() / 2]).is_some()


// ss = (str + str)[1:-1]
// return ss.find(str) != -1



// Runtime3 ms
// Beats
// 83.64%
// Memory2 MB
// Beats
// 81.82%

// 无法理解为什么会tle。
// 。。 s.chars().nth(x).unwrap()  的问题。

// tle ?

// 2,3,5,7  prime part
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.len() == 1 {               // ....
            return false;
        }
        let vu: Vec<usize> = Self::get_divide(s.len());
        
        println!("{:?}", vu);

        let s = s.as_bytes();

        'AAA: 
        for diff in vu {
            println!("start to diff : {}", diff);
            for i in 0..diff {
                let mut idx = i + diff;
                while idx < s.len() {
                    // println!("{} vs {}", idx, i);
                    // if s.chars().nth(idx).unwrap() != s.chars().nth(i).unwrap() {
                    if s[idx] != s[i] {
                        continue 'AAA;
                    }
                    idx += diff;
                }
            }
            return true;
        }

        false
    }

    fn get_divide(sz1: usize) -> Vec<usize> {
        let mut vu: Vec<usize> = Vec::new();
        vu.push(1);             // ....

        let mut vb = vec![false; sz1 / 2 + 1];
        for i in 2..vb.len() {
            if sz1 % i != 0 || vb[i] {
                continue;
            }
            vu.push(i);
            // for j := i + i; j < vb.len(); j += i {
            let mut j = i + i;
            while j < vb.len() {
                vb[j] = false;
                j += i;
            }
        }
        vu.reverse();
        vu
    }
}



struct Solution {}

fn main()
{

    // let s: String = "bacbac".to_string();
    let s: String = "ymqpxhlrnunyfdzrhbasqifpgynkrrefxsnvucfts".to_string();

    println!("size: {}", s.len());
    println!("size2: {}", "a".to_string().len());

    println!("ans: {:?}", Solution::repeated_substring_pattern(s));
}


