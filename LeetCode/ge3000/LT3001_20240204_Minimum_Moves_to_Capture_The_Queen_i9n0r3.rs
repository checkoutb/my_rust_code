





// AC


// Accepted
// 14.2K
// Submissions
// 71.9K
// Acceptance Rate
// 19.8%
// ??? why only 19.8% ?
//
// ... 4,3  3,4  5,2 ...
impl Solution {

    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {

        if e+f==c+d {   // /
            if a+b != c+d || (a < c.min(e) || a > c.max(e)) {
                return 1;
            }
        }
        if e+9-f==c+9-d {       // \
            if a+9-b != c+9-d || (a < c.min(e) || a > c.max(e)) {
                return 1;
            }
        }
        if a==e {       // |
            if c != a || (d < b.min(f) || d > b.max(f)) {
                return 1;
            }
        }
        if b==f {       // -
            if d != b || (c < a.min(e) || c > a.max(e)) {
                return 1;
            }
        }

        return 2;
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


