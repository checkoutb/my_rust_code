



// g



// if no x,y or x==y, ans is [2n-2, 2n-4, ...2, 0]   [2(n-1), 2(n-2),.. 2(n-(n-1)), 2(n-n)]

// house 1,2,3,....100.  x{30},y{60}
// 1-30-60-100
// {<30}-30-60-{>60}
// 1-30-60-50
// x > (30+60)/2 , 1-30-60-x
//      else,   1-x

// st < end
// st <= 30 
//      end >= 45,  st-30-60-end
//      end <= 45,  st-end
// 30<st<=45
//      end >= 60,  st-30-60-end
//      end < 60,   st-30-60-end-st is a circle
//                  set st=30, 30,31,32..59,60,30,   31 edge, 31 point
//          point's count is odd, 31 point * 2 half circle * ( (15 + 1) * 15 / 2 ), 15 is the point count in half circle ( not contain st)
//                          even, need -1, to be an odd, at last + even/2
// st > 45
//      any end (>st), st-end
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let mut ans = [0i64; n];
        let md = (x + y + 1) / 2;

        for i in 1..=n {
            if i <= x {

            } else if i >= md {

            } else {
                // (x, md)
            }
        }


        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


