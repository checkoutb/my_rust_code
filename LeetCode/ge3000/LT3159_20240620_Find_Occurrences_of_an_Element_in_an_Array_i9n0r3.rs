


// D D

// let aa: Vec<usize> = nums.into_iter().enumerate().filter_map(|(idx, a)| {
//     if a == x {
//         Some(idx)
//     }else {
//         None
//     }
// }).collect();
// let mut r = Vec::new();
// for q in queries {
//     let qq = (q - 1)  as usize;
//     let v = aa.get(qq).map(|x| *x as i32).unwrap_or(-1);
//     r.push(v);
// }
// r


// pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
//     let indices: Vec<i32> = nums.iter().enumerate().filter(|(_, &y)| y == x).map(|(i, _)| i as i32).collect();
//     queries.iter().map(|&y| *indices.get(y as usize - 1).unwrap_or(&-1)).collect()
// }




// Runtime
// 49ms
// Beats55.93%
// Analyze Complexity
// Memory
// 3.66MB
// Beats96.61%
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut xidx = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            if num == x {
                xidx.push(i);
            }
        }
        let mut vi = vec![-1i32; queries.len()];
        for (i, q) in queries.into_iter().enumerate() {
            if (q as usize) <= xidx.len() {
                vi[i] = xidx[q as usize - 1] as i32;
            }
        }
        vi
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


