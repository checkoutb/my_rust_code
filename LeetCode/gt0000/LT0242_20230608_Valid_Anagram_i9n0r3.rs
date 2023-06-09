
// use itertools::Itertools;


// D D

// if s.len() != t.len() {
//     return false;
// }
// let mut map = std::collections::HashMap::new();
// s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
// t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
// map.into_values().all(|v: i32| v == 0)


// use std::collections::HashMap;
//
// fn count_chars(s: String) -> HashMap::<char, i32> {
//     s.chars().fold(HashMap::<char, i32>::new(), |mut map, ch| {
//         map.entry(ch).and_modify(|count| *count += 1 ).or_insert(1);
//         map
//     })
// }
//
// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         count_chars(s) == count_chars(t)
//     }
// }



// pub fn is_anagram(s: String, t: String) -> bool {
//     s.bytes().fold([0; 26], |mut counter, b| { counter[(b - 97) as usize] += 1; counter }) 
//         == t.bytes().fold([0; 26], |mut counter, b| { counter[(b - 97) as usize] += 1; counter })
// }


// let mut s = s.chars().fold([0; 31], |mut acc, ch| {
//     acc[ch as usize - 97] += 1;
//     acc
// });
// t.chars().for_each(|ch| s[ch as usize - 97] -= 1);
// s.iter().all(|e| e == &0)



// pub fn is_anagram(s: String, t: String) -> bool {
//     let counter1 = Solution::count(s);
//     let counter2 = Solution::count(t);
//     counter1 == counter2
// }
// fn count(s: String) -> HashMap<char, u32> {
//     let mut counter: HashMap<char, u32> = HashMap::new();
//     s.chars().for_each(|c| { 
//         counter.insert(c, counter.get(&c).unwrap_or(&0) + 1); 
//     });
//     counter
// }




// Runtime7 ms
// Beats
// 25.61%
// Memory2.5 MB
// Beats
// 13.73%
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // let arr1 = s.as_bytes().sort();
        // let arr2 = t.as_bytes().sort();
        // let mut arr1 = unsafe {s.as_bytes_mut()};
        // let mut arr2 = unsafe {t.as_bytes_mut()};
        // let arr1 = s.chars().sorted().collect::<String>().as_bytes();        // need itertools::Itertools
        // let arr2 = t.chars().sorted().collect::<String>().as_bytes();
        // arr1.sort();
        // arr2.sort();

        // let mut vc: Vec<char> = s.chars().collect();
        // vc.sort_unstable();
        // //let arr1 = vc.into_iter().collect::<String>().as_bytes();
        // // let t2 = vc.into_iter().collect::<String>();
        // // let arr1 = vc.into_iter().collect
        // vc = t.chars().collect();
        // vc.sort_unstable();
        // let arr2 = vc.into_iter().collect::<String>().as_bytes();
        
        if s.len() != t.len() {
            return false;
        }

        let mut arr1: Vec<char> = s.chars().collect();
        arr1.sort();
        let mut arr2: Vec<char> = t.chars().collect();
        arr2.sort();

        for i in 0..arr1.len() {
            if arr1[i] != arr2[i] {
                return false;
            }
        }
        true
    }
}






struct Solution {}

fn main()
{

    let s = "anagram".to_string();
    let t = "nagaram".to_string();

    println!("ans: {:?}", Solution::is_anagram(s, t));
}


