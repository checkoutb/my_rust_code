






impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut vc: Vec<char> = s.chars().collect();
        let k: usize = k as usize;
        let mut idx: usize = 0;
        while idx < s.len() {
            let mut en: usize = (idx + k).min(s.len()) - 1;
            let mut st: usize = idx;
            while st < en {
                vc.swap(st, en);
                st += 1;
                en -= 1;
            }
            idx = idx + (k as usize) * 2;
        }
        vc.iter().collect::<String>()
    }
}




struct Solution {}

fn main()
{
    let s = "abcdefg".to_string();
    let k = 2;

    println!("ans: {:?}", Solution::reverse_str(s, k));
}


