

// D D

// while s_idx >=0 || t_idx >= 0{
//     while s_idx >= 0{




// Runtime1 ms
// Beats
// 74.42%
// Memory1.9 MB
// Beats
// 93.2%

// back -> front
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut si = s.len() as i32;
        let mut ti = t.len() as i32;

        while si > 0 && ti > 0 {
            si -= 1;
            ti -= 1;
            let mut cnt = 0i32;
            while si >= 0 {
                if s[si as usize] == b'#' {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt == -1 {
                    break;
                }
                si -= 1;
            }
            cnt = 0;
            while ti >= 0 {
                if t[ti as usize] == b'#' {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt == -1 {
                    break;
                }
                ti -= 1;
            }
            if si >= 0 && ti >= 0 {
                if s[si as usize] != t[ti as usize] {
                    return false;
                }
            } else if si == -1 && ti == -1 {
                ;
            } else {
                return false;
            }
        }
        // println!("{}, {}", si, ti);
        if si > 0 {
            si -= 1;
            let mut cnt = 0i32;
            while si >= 0 {
                if s[si as usize] == b'#' {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt == -1 {
                    return false;
                }
                si -= 1;
            }
        }
        if ti > 0 {
            ti -= 1;
            let mut cnt = 0i32;
            while ti >= 0 {
                if t[ti as usize] == b'#' {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt == -1 {
                    return false;
                }
                ti -= 1;
            }
        }
        return true;
    }
}




struct Solution {}

fn main()
{

    // let s = String::from("bxj##tw");
    // let t = String::from("bxj###tw");

    // let s = String::from("ab#c");
    // let t = String::from("ac#c");

    // let s = "bbbextm".to_string();
    // let t = "bbb#extm".to_string();

    let s = "nzp#o#g".to_string();
    let t = "b#nzp#o#g".to_string();

    println!("ans: {:?}", Solution::backspace_compare(s, t));
}


