








impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut piece = Piece{ cnt: 0, arr: vec![0; 123] };
        piece.init(t);

        let s2 = s.clone().into_bytes();
        let mut st = 0usize;
        let mut ans = String::new();
        for (idx, ch) in s2.clone().into_iter().enumerate() {
            piece.add(ch);
            while piece.right() {
                if ans.is_empty() || idx - st + 1 < ans.len() {
                    ans = String::from_utf8(s2[st..idx + 1].to_vec()).unwrap();
                }
                piece.remove(s2[st]);
                st += 1;
            }
        }
        ans
    }
}

struct Solution {}



struct Piece {
    cnt: i32,       // still not include
    arr: Vec<i32>,  // t's char count
    // arr2: vec![26],
}
impl Piece {
    fn init(&mut self, t: String) {
        let t = t.into_bytes();
        // let au8 = b'a';
        for i in t {
            // self.arr[(i - au8) as usize] += 1;
            self.arr[i as usize] += 1;
        }
        for i in &self.arr {
            if *i != 0 {
                self.cnt += 1;
            }
        }
        println!("  {:?},  {:?}  ", self.cnt, self.arr)
    }
    fn add(&mut self, ch: u8) {
        self.arr[ch as usize] -= 1;
        if self.arr[ch as usize] == 0 {
            self.cnt -= 1;
        }
    }
    fn remove(&mut self, ch: u8) {
        self.arr[ch as usize] += 1;
        if self.arr[ch as usize] == 1 {
            self.cnt += 1;
        }
    }
    fn right(&mut self) -> bool {
        self.cnt == 0
    }
}

fn main()
{

    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();

    println!("ans: {:?}", Solution::min_window(s, t));
}


