

// D D

// for {for {}} 硬算就可以了。。。

// 保存 边长。 返回的时候 边长*边长





// Runtime
// 79ms
// Beats45.83%of users with Rust
// Memory
// 2.26MB
// Beats83.33%of users with Rust

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {

        let mut ans = 0i64;
        let mut vp = Vec::new();
        for i in 0..top_right.len() {
            vp.push((bottom_left[i][0], bottom_left[i][1], top_right[i][0], top_right[i][1], i as i32));
        }
        vp.sort_by_key(|p| p.0);
        let mut vp2 = vp.clone();
        vp2.sort_by_key(|p| p.2);
        let mut st = 0usize;
        for i in 0..vp.len() {
            while vp2[st].2 <= vp[i].0 {
                st += 1;
            }
            let (x1, y1, x2, y2, idx) = vp[i];
            for j in st..vp2.len() {
                let tx = vp2[j].3.min(y2) - vp2[j].1.max(y1);
                let ty = x2.min(vp2[j].2) - x1.max(vp2[j].0);
                if tx <= 0 || ty <= 0 {
                    continue;
                }
                let mut t2 = tx.min(ty) as i64;
                t2 *= t2;
                if idx != vp2[j].4 && t2 > ans {
                    ans = t2;
                }
            }
        }
        
        ans

        // let (mut t, mut b, mut l, mut r) = (top_right[0][1], bottom_left[0][1], bottom_left[0][0], top_right[0][0]);     // top, bottom, left, right
        // for i in 1..top_right.len() {
        //     t = t.min(top_right[i][1]);
        //     b = b.max(bottom_left[i][1]);
        //     l = l.max(bottom_left[i][0]);
        //     r = r.min(top_right[i][0]);
        // }
        // if t <= b || l >= r {
        //     0
        // } else {
        //     let t2 = (t - b).min(r - l) as i64;
        //     t2 * t2
        // }
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


