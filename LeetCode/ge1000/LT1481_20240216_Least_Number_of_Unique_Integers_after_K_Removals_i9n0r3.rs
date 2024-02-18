


// D D


// for e in arr {
//     count.entry(e).and_modify(|c| *c += 1).or_insert(1);
// }

// let mut vec: Vec<(i32, usize)> = count.iter().map(|(n, c)| (*n, *c)).collect();


// Runtime
// 20ms
// Beats70.00%of users with Rust
// Memory
// 5.10MB
// Beats80.00%of users with Rust

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map2 : HashMap<i32, i32> = HashMap::new();

        for (_, n) in arr.into_iter().enumerate() {
            map2.insert(n, map2.get(&n).unwrap_or(&0) + 1);
        }

        // let mut vi = vec![0; map2.len()];
        let mut vi = Vec::new();

        for (_, v) in map2.into_iter() {
            vi.push(v);
        }

        vi.sort_unstable();
        
        let mut ans = vi.len() as i32;
        let mut k = k;

        for n in vi {
            if n <= k {
                ans -= 1;
                k -= n;
            } else {
                break;
            }
        }
        ans
    }
}

struct Solution {}

fn main()
{

    let vi = [5,5,3].to_vec();
    let k = 1;

    println!("ans: {:?}", Solution::find_least_num_of_unique_ints(vi, k));
}


