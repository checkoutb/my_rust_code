


// Runtime
// 64ms
// Beats50.00%
// Memory
// 11.28MB
// Beats-%




// k 个 相同长度的 substr

// hash?
// ...rust的好像不好写。。  。。splite_off  还有一个 split_at

use std::collections::HashMap;
impl Solution {

    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let mut map2: HashMap<String, i32> = HashMap::new();
        let subsz = s.len() as i32 / k;
        Self::splite_to_map(s, subsz, 1, &mut map2);
        Self::splite_to_map(t, subsz, -1, &mut map2);

        map2.into_values().all(|x| x == 0)
    }

    fn splite_to_map(mut s: String, subsz: i32, added: i32, map2: &mut HashMap<String, i32>) {

        let mut sp_at = s.len() - subsz as usize;
        loop {
            let subs = s.split_off(sp_at);
            map2.entry(subs).and_modify(|val| *val += added).or_insert(added);
            if sp_at == 0 {
                break;
            } else {
                sp_at -= subsz as usize;
            }
        }
    }
}


struct Solution {}

fn main()
{

    let s = "abcd".to_string();
    let t = "cdab".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::is_possible_to_rearrange(s, t, k));
}


