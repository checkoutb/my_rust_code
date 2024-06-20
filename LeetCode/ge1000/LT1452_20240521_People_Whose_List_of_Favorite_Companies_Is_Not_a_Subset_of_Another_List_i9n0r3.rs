

// D D

// let mut f_sets: Vec<HashSet<String>> = Vec::new();
// for i in 0..n {
//     let temp: HashSet<String> = favorite_companies[i].iter().cloned().collect();
//     f_sets.push(temp.clone());
// }
//
// for i in 0..n {
//     let mut in_sub: bool = false;
//     for j in 0..n {
//         if i != j && f_sets[i].is_subset(&f_sets[j]) {
//             in_sub = true;
//         }
//     }

// is_subset


// C++ includes

// uf


// Runtime
// 31ms
// Beats100.00%of users with Rust
// Memory
// 10.04MB
// Beats66.67%of users with Rust

impl Solution {
    // 100 person
    // 500 favorite / person
    // 20 company name
    // 
    // std::bitset(no,maybe there are 100 * 500 companys..) or just std::set ?
    // es的倒排
    // 人数是确定的，所以可以 bitset<100>
    // map<company name, bitset<100>>
    // 
    // xx，这里是rust。。。there is no bitset...
    // (i64, i64)
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map2: HashMap<String, (i64, i64)> = HashMap::new();

        // favorite_companies.sort_by_key(|a| -a.len());
        let mut ans = Vec::new();
        for i in 0..favorite_companies.len() {
            for cname in favorite_companies[i].iter() {
                // let a = map2.get(cname).unwrap_or_default();
                map2.entry(cname.to_string()).and_modify(|a| *a = Self::set_bit(*a, i)).or_insert(Self::set_bit((0,0), i));
            }
        }

        for i in 0..favorite_companies.len() {
            let mut a = (i64::MAX, i64::MAX);
            for cname in favorite_companies[i].iter() {
                a = Self::and_bit(a, *map2.get(cname).unwrap());
            }
            if Self::cnt_bit(a) == 1 {
                ans.push(i as i32);
            }
        }

        ans
    }

    fn set_bit(mut a: (i64, i64), idx: usize) -> (i64, i64) {
        if idx > 60 {
            a.0 |= 1 << (idx - 61);
        } else {
            a.1 |= 1 << idx;
        }
        a
    }

    // fn set_bit(a: i64, b: i64, idx: usize) -> (i64, i64) {
    //     if idx > 60 {
    //         a |= 1 << (idx - 61);
    //     } else {
    //         b |= 1 << idx
    //     }
    //     (a, b)
    // }

    fn and_bit(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
        (a.0 & b.0, a.1 & b.1)
    }

    fn cnt_bit(mut a: (i64, i64)) -> i32 {
        let mut cnt = 0;
        while a.0 != 0 {
            cnt += 1;
            a.0 &= a.0 - 1;
        }
        while a.1 != 0 {
            cnt += 1;
            a.1 &= a.1 - 1;
        }
        cnt
    }
}


struct Solution {}

fn main()
{
    let vvs = [["l".to_string(), "g".to_string(), "f".to_string()].to_vec(), 
    ["l".to_string(), "a".to_string()].to_vec(), ["f".to_string(),"g".to_string()].to_vec()].to_vec();

    println!("ans: {:?}", Solution::people_indexes(vvs));
}


