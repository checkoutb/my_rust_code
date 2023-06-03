
use std::collections::HashMap;


// D D

//     let mut frequency = [0; 256];
//     for b in s.bytes() {
//         frequency[b as usize] += 1;
//     }
//
//     s.clear();
//     while let Some(i) = max(&frequency) {
//         let times = frequency[i];
//         for _ in 0..times {
//             s.push(i as u8 as char);
//         }
//         frequency[i] = 0;
//     }
//
// fn max(a: &[i32]) -> Option<usize> {
//     let mut max = 0;
//     let mut index = None;
//     for (i, &num) in a.iter().enumerate().filter(|(_, &c)| c != 0) {
//         if num > max {
//             index = Some(i);
//             max = num;
//         }
//     }
//     index
// }


// use std::collections::HashMap;
// let mut map: HashMap<char, i32> = HashMap::new();
// for c in s.chars() {
//     *map.entry(c).or_insert(0) += 1;
// }
// let mut v = map.into_iter().collect::<Vec<_>>();
// v.sort_by(|a, b| b.1.cmp(&a.1));
// let mut res = String::new();
// for (c, n) in v {
//     for _ in 0..n {
//         res.push(c);
//     }
// }
// res




// Runtime117 ms
// Beats
// 8%
// Memory3 MB
// Beats
// 8%

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map2: HashMap<char, i32> = HashMap::new();

        // for ch in s.char_indices() {
        for ch in s.chars().collect::<Vec<_>>() {
            match map2.get(&ch) {
                //Some(v) => *v = *v + 1,
                Some(v) => map2.insert(ch, *v + 1),             // as char  is error
                None => map2.insert(ch, 1)
            };
            // if map2.contains_key(&ch) {
            //     map2.insert(ch, map2[ch] + 1)
            // } else {
            //     map2.insert(ch, 1)
            // }
        }

        // let mut s = s;
        // s.chars().sort_by(|a, b| map2[a] > map2[b]);
        
        let mut chs: Vec<char> = s.chars().collect();
        
        //chs.sort_by(|a, b| map2[a] > map2[b]);
        chs.sort_by(|a, b| if map2[a] == map2[b] {a.cmp(&b)} else {map2[b].cmp(&map2[a])});

        //s = String::from_raw_points(chs).unwrap();
        let s = chs.iter().collect();

        s

        // let mut vi: Vec<i32> = vec![0; 123];
        // for ch in s.char_indices() {
        //     vi[ch as usize]++
        // }
        // for i in 0..vi.len {
        //     if vi[i] != 0 {

        //     }
        // }
    }
}


struct Solution {}

fn main()
{

    // let s = "tree";
    //let s = "Aabb";
    let s = "loveleetcode";


    println!("ans: {:?}", Solution::frequency_sort((&s).to_string()));
}


