


// D D


// let mut result = Vec::new();
// let mut group_cache = HashMap::new();
//
// for (idx, group_size) in group_sizes.into_iter().enumerate() {
//     let group = group_cache.entry(group_size).or_insert(Vec::new());
//     group.push(idx as i32);
//     if group.len() as i32 == group_size {
//         result.push(std::mem::replace(group, Vec::new()));
//     }
// }




// Runtime0 ms
// Beats
// 100%
// Memory2.2 MB
// Beats
// 53.33%
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map2 : HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..group_sizes.len() {
            map2.entry(group_sizes[i]).or_insert(Vec::new()).push(i as i32);
        }
        let mut vvi = Vec::new();
        let mut v2 = Vec::new();
        for (sz, vi) in &map2 {
            v2 = Vec::new();
            for i in 0..vi.len() {
                v2.push(vi[i]);
                if v2.len() == *sz as usize {
                    vvi.push(v2);
                    v2 = Vec::new();
                }
            }
        }
        vvi
    }
}


struct Solution {}

fn main()
{

    let vi = [3,3,3,3,3,1,3].to_vec();

    println!("ans: {:?}", Solution::group_the_people(vi));
}


