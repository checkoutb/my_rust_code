





// Runtime2 ms
// Beats
// 65.52%
// Memory2.1 MB
// Beats
// 24.14%

// n+2 水平， m+2 垂直
impl Solution {

    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort();
        v_bars.sort();

        let mut mxh = 2i32;     // bars.len >= 1
        let mut mxv = 2i32;
        
        let mut t2 = 2i32;
        
        for i in 1..h_bars.len() {
            if h_bars[i] == h_bars[i - 1] + 1 {
                t2 += 1;
            } else {
                mxh = mxh.max(t2);
                t2 = 2;
            }
        }
        mxh = mxh.max(t2);

        t2 = 2;
        for i in 1..v_bars.len() {
            if v_bars[i] == v_bars[i - 1] + 1 {
                t2 += 1;
            } else {
                mxv = mxv.max(t2);
                t2 = 2;
            }
        }
        mxv = mxv.max(t2);
        t2 = mxh.min(mxv);
        t2 * t2
    }

    // error ... 
    pub fn err_maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;

        h_bars.sort();
        v_bars.sort();

        let mut mxh = 1i32;
        let mut mxv = 1i32;
        let mut st = 1i32;
        if !h_bars.is_empty() {
            st = h_bars[0] - 1;
        }
        
        let mut t2 = st;
        for bar in h_bars {
            if bar != t2 + 1 {
                mxh = mxh.max(bar - st - 1);
                st = bar;
                t2 = st;
            } else {
                t2 = bar;
            }
        }
        // mxh = mxh.max(n + 2 - st);

        st = 1;
        if !v_bars.is_empty() {
            st = v_bars[0] - 1;
        }
        t2 = st;
        for bar in v_bars {
            if bar != t2 + 1 {
                // println!("{},{},{}",bar, t2, st);
                mxv = mxv.max(bar - st - 1);
                st = bar;
                t2 = st;
            } else {
                t2 = bar;
            }
        }
        // println!("{}", mxv);
        // mxv = mxv.max(m + 2 - st);

        println!("{},{}", mxh, mxv);

        t2 = mxh.min(mxv);
        t2 * t2
    }
}



struct Solution {}

fn main()
{

    // let n = 2;
    // let m = 1;
    // let vi = [2,3].to_vec();
    // let v2 = [2].to_vec();

    // let n = 1;
    // let m = 1;
    // let vi = [2].to_vec();
    // let v2 = [2].to_vec();

    let n = 2;
    let m = 3;
    let vi = [2,3].to_vec();
    let v2 = [2,3,4].to_vec();

    // let n = 2;
    // let m = 4;
    // let vi = [3,2].to_vec();
    // let v2 = [2,4].to_vec();

    // let n = 14;
    // let m = 4;
    // let vi = [13].to_vec();
    // let v2 = [3,4,5,2].to_vec();



    println!("ans: {:?}", Solution::maximize_square_hole_area(n,m,vi,v2));
}


