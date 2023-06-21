


// D D

// return n == 192 || n == 219 || n == 273 || n == 327;



// Runtime4 ms
// Beats
// 27.8%
// Memory2 MB
// Beats
// 91.67%

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        if n * 3 >= 1000 {
            return false;
        }
        let t2 = (n * 1000 + n * 2) * 1000 + n * 3;
        let mut t2 = t2 as usize;
        // println!("{}", t2);
        let mut vb = vec![false; 10];
        vb[0] = true;
        while t2 > 0 {
            if vb[t2 % 10] {
                return false;
            }
            vb[t2 % 10] = true;
            t2 /= 10;
        }
        vb.iter().all(|x| *x)
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::is_fascinating(192));
}


