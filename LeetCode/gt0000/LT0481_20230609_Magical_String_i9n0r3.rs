
// D D

// num = 3 - num




// Runtime3 ms
// Beats
// 35.71%
// Memory2.6 MB
// Beats
// 57.14%

// 1 22 11 2 1 22 1 22 11 2 11 22
// 1 22 11 2 1 22 1 22 11 2 11 22 1 2 11 2 1 22 11 2

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 3 {
            return 1;
        }
        let mut vi: Vec<i32> = [1,2,2].to_vec();
        let mut idx: usize = 2;
        let mut t2: i32 = 1;
        let mut ans: i32 = 1;
        while vi.len() < n as usize {
            if vi[idx] == 1 {
                vi.push(t2);
                if t2 == 1 {
                    ans += 1;
                }
            } else {
                vi.push(t2);
                if t2 == 1 {
                    ans += 1;
                }
                if vi.len() < n as usize {
                    vi.push(t2);
                    if t2 == 1 {
                        ans += 1;
                    }
                }
            }
            idx += 1;
            t2 = 1 + t2 % 2;
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


