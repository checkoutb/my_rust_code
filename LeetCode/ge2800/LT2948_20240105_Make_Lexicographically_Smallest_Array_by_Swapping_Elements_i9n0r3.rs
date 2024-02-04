

// D D

// let mut pair: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
// pair.sort_by_key(|x| x.0);


// let mut start = 0;
// for end in 0..size {
//     if end + 1 >= size || pair[end + 1].0 - pair[end].0 > limit {
//         let temp: Vec<usize> = (start..=end).map(|i| pair[i].1).collect();


// let mut b: Vec<(i32, usize)> = Vec::new();
// let n = nums.len();
// for i in 0..n {
//     b.push((nums[i], i));
// }


// Runtime76 ms
// Beats
// 81.25%
// Memory5.8 MB
// Beats
// 87.50%

// da ->
// |val - val| < limit. split to group. sort each group
impl Solution {

    // struct Pair(i32, usize);

    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut vp = Vec::new();
        for (i, num) in nums.clone().into_iter().enumerate() {
            vp.push((num, i));
        }

        vp.sort_by(|a, b| if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        });

        let mut idx = 0;
        while idx < vp.len() {
            let mut t2 = idx;
            let mut vi = vec![vp[t2].1 ; 1];

            while t2 + 1 < vp.len() && (vp[t2 + 1].0 - vp[t2].0).abs() <= limit {
                t2 += 1;
                vi.push(vp[t2].1);  // after t2+=1
            }
            vi.sort();

            for i in 0..vi.len() {
                nums[vi[i]] = vp[idx + i].0;
            }

            idx = t2 + 1;
        }
        nums
    }
}


struct Solution {}

fn main()
{
    let vi = [9,8,3,5,1].to_vec();
    let lmt = 2;

    println!("ans: {:?}", Solution::lexicographically_smallest_array(vi, lmt));
}


