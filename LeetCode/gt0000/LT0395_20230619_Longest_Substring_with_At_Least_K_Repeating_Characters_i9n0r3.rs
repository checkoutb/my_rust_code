






// Runtime1 ms
// Beats
// 76.92%
// Memory2 MB
// Beats
// 84.62%
impl Solution {

    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::dfsa1(&s, k as usize, 0, s.len())
    }

    // [st, en)
    fn dfsa1(s: &String, k: usize, st: usize, en: usize) -> i32 {
        if st + k > en {
            return 0;
        }
        let mut vi = vec![0; 26];
        let au8: u8 = b"a"[0];
        for i in st..en {
            vi[(s.as_bytes()[i] - au8) as usize] += 1;
        }
        if vi.iter().all(|&x| x >= k || x == 0) {
            return (en - st) as i32;
        }
        let mut st2 = st;
        let mut ans = 0;
        for i in st..en {
            if vi[(s.as_bytes()[i] - au8) as usize] < k {
                ans = ans.max(Self::dfsa1(s, k, st2, i));
                st2 = i + 1;
            }
        }
        ans = ans.max(Self::dfsa1(s, k, st2, en));          // ....
        ans
    }

    // wo qiao, not == k ...
    pub fn longest_substring_error(s: String, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut map2: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut vi: Vec<i32> = vec![0; 26];
        map2.insert(vi.clone(), -1);
        let mut ans = 0;
        let au8: u8 = b"a"[0];
        // let mut idx: usize = 0;      // warn: never read
        // for ch in s.as_bytes() {        // u8
        for i in 0..s.len() {
            let ch = s.as_bytes()[i];
            let i = i as i32;
            let idx = (ch - au8) as usize;
            vi[idx] = (vi[idx] + 1) % k;
            if map2.contains_key(&vi) {
                ans = ans.max(i - map2.get(&vi).unwrap());
            } else {
                map2.insert(vi.clone(), i);
            }
        }
        ans
    }
}





struct Solution {}

fn main()
{

    // let s = "aaabb".to_string();
    // let k = 3;

    let s = "ababbc".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::longest_substring(s, k));
}


