




// Runtime
// 8ms
// Beats75.00%of users with Rust
// Memory
// 2.63MB
// Beats42.31%of users with Rust

// let v: Vec<_> = "ababa".match_indices("aba").collect();
// assert_eq!(v, [(0, "aba")]); // only the first `aba`

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
        // let vsa = s.matche_indices(a).collect();
        // let vsb = s.matche_indices(b).collect();

        let va = Self::get_index(&s, &a);
        let vb = Self::get_index(&s, &b);

        let mut vi = Vec::new();

        let mut stb = 0usize;

        for i in 0..va.len() {
            while stb < vb.len() && vb[stb] < va[i] {
                stb += 1;
            }
            if stb > 0 && (va[i] - vb[stb - 1]).abs() <= k {            // > 0...
                vi.push(va[i]);
            } else if stb < vb.len() && (va[i] - vb[stb]).abs() <= k {
                vi.push(va[i]);
            }
        }

        vi
    }

    fn get_index(s: &String, a: &String) -> Vec<i32> {
        let mut vi = vec![];
        let mut st = 0usize;
        loop {
            if let Some(idx) = s[st..].find(a) {
                vi.push((idx + st) as i32);
                st = st + idx + 1;      // st = idx + 1....
                // println!(" .  {:?}, {:?}", st, idx);
            } else {
                break;
            }
        }
        vi
    }
}


struct Solution {}

fn main()
{

    // let s = "isawsquirrelnearmysquirrelhouseohmy".to_string();
    // let a = "my".to_string();
    // let b = "squirrel".to_string();
    // let k = 15;

    let s = "abcd".to_string();
    let a = "a".to_string();
    let b = String::from("a");
    let k = 4;

    println!("ans: {:?}", Solution::beautiful_indices(s, a, b, k));

}


