

// D D

// use std::cmp;
// use std::collections::HashMap;
// impl Solution {
//     pub fn minimum_seconds(mut nums: Vec<i32>) -> i32 {
//         nums.append(&mut nums.clone());
//         let mut m: HashMap<i32, (i32, usize)> = HashMap::new();
//         for i in 0..nums.len() {
//             m.entry(nums[i])
//                 .and_modify(|e| {
//                     e.0 = cmp::max(e.0, (i - e.1) as i32);
//                     e.1 = i;
//                 })
//                 .or_insert((0,i));
//         }
//         let mut ans = 1000000;
//         for i in m.values(){
//             ans = cmp::min(ans, i.0);
//         }
//         ans/2
//     }
// }




// Runtime29 ms
// Beats
// 100%
// Memory6.9 MB
// Beats
// 95%

// suanfa jiandan, yuyan nan...

// kanqilai hen jiandan, what u need is just a idea
// 1% de linggan
impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map2: HashMap<i32, usize> = HashMap::new(); // last appear
        let mut map3: HashMap<i32, i32> = HashMap::new();   // <num : mx sec>

        for i in 0..nums.len() {
            map2.insert(nums[i], i);
        }

        for i in 0..nums.len() {
            if let Some(lst) = map2.get(&nums[i]) {
                if i == *lst {
                    continue;
                }

                let mut tm = 0;
                if *lst > i {
                    tm = ((i + nums.len() - *lst) / 2) as i32;
                } else {
                    tm = ((i - *lst) / 2) as i32;
                }
                map3.insert(nums[i], tm.max(*map3.get(&nums[i]).unwrap_or(&0)));
                map2.insert(nums[i], i);
                // if (map3.contains_key(nums[i])) {
                //     map3.insert(nums[i], tm.max())
                // } else {
                //     map3.insert(nums[i], tm);
                // }
            }
        }

        // println!("{:?}", map2);
        // println!("{:?}", map3);

        let mut ans = (nums.len() / 2) as i32;
        for (_k, v) in map3.iter() {
            ans = ans.min(*v);
        }
        return ans as i32;
    }
}




struct Solution {}

fn main()
{

    // let vi = [2,1,3,3,2].to_vec();
    let vi = [2,2,2,2].to_vec();

    println!("ans: {:?}", Solution::minimum_seconds(vi));
}


