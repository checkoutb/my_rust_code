




// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.23MB
// Beats62.12%



// [0, *] first appear,  [*, end] last appear
// 没用过

impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        
        let sidx = p.find('*').unwrap();

        let t2 = s.find(&p[0..sidx]);
        let t3 = s.rfind(&p[sidx + 1..]);

        // println!("{:?} ... {:?}  {:?}", &p[0..sidx], &p[sidx + 1..], &p[..sidx]);
        // println!("{:?}, {:?}", t2, t3);
        if t2.is_some() && t3.is_some() {
            return t2.unwrap() + sidx <= t3.unwrap();
        } else {
            return false;
        }

    }
}



struct Solution {}

fn main()
{
    let s = "leeae".to_string();
    let p = "ee*e".to_string();

    println!("ans: {:?}", Solution::has_match(s, p));
}


