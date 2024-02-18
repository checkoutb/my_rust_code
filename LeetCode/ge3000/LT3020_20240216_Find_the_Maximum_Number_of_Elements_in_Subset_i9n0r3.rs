
// D D

// for x in nums{
//     *cnt.entry(x).or_insert(0)+=1;
// }

// if let Some(&v)=cnt.get(&1){
//     ans=v-(v%2^1);
//     cnt.remove(&1);
// }

// while *cnt.get(&k).unwrap_or(&0)>1{
//     count+=2;
//     k*=k;
// }

// count+=if cnt.contains_key(&k){1}else{-1};


// Runtime
// 42ms
// Beats52.38%of users with Rust
// Memory
// 4.28MB
// Beats60.32%of users with Rust

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut map2 = std::collections::HashMap::new();
        for n in nums {
            map2.entry(n).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        let mut vi: Vec<i32> = map2.iter().map(|(n, _c)| *n).collect();
        // println!("{:?}", vi);
        vi.sort();
        vi.reverse();
        let mut ans = 1;
        let mut t2;
        if vi[vi.len() - 1] == 1 {
            t2 = *map2.get(&1).unwrap();
            if t2 % 2 == 0 {
                t2 -= 1;
            }
            ans = t2;
            vi.pop();
        }
        let mut t3;
        const SQRT : i32 = 31622;     // sqrt(10^9)
        let mut map3 = std::collections::HashMap::new();
        for (_, n) in vi.into_iter().enumerate() {
            t3 = *map2.get(&n).unwrap();
            if t3 == 1 {
                map3.insert(n, 1);
                continue;
            }
            if n > SQRT {
                t2 = 0;
            } else {
                t2 = *map3.get(&(n*n)).unwrap_or(&0);
            }
            if t2 == 0 {
                map3.insert(n, 1);
            } else {
                map3.insert(n, t2 + 2);
                ans = ans.max(t2 + 2);
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    // let vi = [1,2,2,4,5].to_vec();
    // let vi = [1,1].to_vec();
    let vi = [1,16,49,16,121].to_vec();

    println!("ans: {:?}", Solution::maximum_length(vi));
}


