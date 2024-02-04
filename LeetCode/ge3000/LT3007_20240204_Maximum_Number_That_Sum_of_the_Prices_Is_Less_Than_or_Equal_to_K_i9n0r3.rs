


// D

// BS



// g



impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let mut k = k;
        let mut vl = vec![0; 53];
        for i in 1..=52 {
            if i % x == 0 {
                vl[i as usize] = vl[i as usize - 1] * 2 + (1<<(i) - 1);
            } else {
                vl[i as usize] = vl[i as usize - 1] * 2;
            }
        }
        let mut ans = 0i64;

        println!("{:?}  {:?}", vl, k);

        for i in 1..=52 {
            let t2 = 53 - i;
            if vl[t2] <= k {
                ans |= 1 << (t2);
                k -= vl[i];
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    let k = 9;
    let x = 1;

    println!("ans: {:?}", Solution::find_maximum_number(k, x));
}


