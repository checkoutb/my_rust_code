







// Runtime
// 1 ms
// Beats
// 61.90%
// Memory
// 2.4 MB
// Beats
// 57.14%
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let mut idx1: i32 = -1;
        let mut idx2: i32 = -1;
        let mut arr: Vec<i32> = vec![0; 26];
        let vc: Vec<u8> = s.as_bytes().to_vec();
        let vc2: Vec<u8> = goal.as_bytes().to_vec();
        let au8: u8 = b"a"[0];

        for i in 0..vc.len() {
            if vc[i] != vc2[i] {
                if idx1 == -1 {
                    idx1 = i as i32;
                } else if idx2 == -1 {
                    idx2 = i as i32;
                } else {
                    return false;
                }
            } else {
                arr[(vc[i] - au8) as usize] += 1;
            }
        }
        
        // println!("{}, {}", idx1, idx2);
        if idx1 != -1 && idx2 != -1 {
            return vc[idx1 as usize] == vc2[idx2 as usize] && vc[idx2 as usize] == vc2[idx1 as usize];
        } else if idx1 != -1 {
            return false;
        } else {
            for i in 0..arr.len() {
                if arr[i] > 1 {
                    return true;
                }
            }
        }
        return false;
    }
}


struct Solution {}

fn main()
{

    let s = "ab".to_string();
    let t = "ba".to_string();


    println!("ans: {:?}", Solution::buddy_strings(s, t));
}


