

// int second_min_sz = INT_MAX;
// vector<int> min_sz(26, INT_MAX);
// for (int i = 0; i < ps.size(); ++i) {
//     int sz = max(abs(ps[i][0]), abs(ps[i][1]));
//     if (min_sz[s[i] - 'a'] > sz)
//         swap(min_sz[s[i] - 'a'], sz);
//     second_min_sz = min(second_min_sz, sz);
// }
// return ranges::count_if(min_sz, [&](int sz){ return sz < second_min_sz; });

// vi + int



// Runtime
// 27ms
// Beats100.00%of users with Rust
// Memory
// 9.94MB
// Beats100.00%of users with Rust

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut vi = vec![vec![i32::MAX; 2]; 123];
        let mut tmn = 0i32;
        let mut t2 = 0usize;
        let mut siter = s.chars();
        for i in 0..points.len() {
            tmn = points[i][0].abs().max(points[i][1].abs());       // max
            // t2 = s[i] as usize;
            t2 = siter.next().unwrap() as usize;
            if tmn <= vi[t2][0] {
                vi[t2][1] = vi[t2][0];
                vi[t2][0] = tmn;
            } else if tmn < vi[t2][1] {
                vi[t2][1] = tmn;
            }
        }
        let mut t2 = i32::MAX;
        for i in ('a' as usize) ..= ('z' as usize) {
            t2 = t2.min(vi[i][1] - 1);
        }

        let mut ans = 0i32;
        for i in 0..points.len() {
            if points[i][0].abs().max(points[i][1].abs()) <= t2 {
                ans += 1;
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


