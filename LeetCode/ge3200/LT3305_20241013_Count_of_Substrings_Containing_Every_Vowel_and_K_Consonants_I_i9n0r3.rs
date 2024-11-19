



// II tle
// 估计是 chars.nth的问题。  再说了。


// Runtime
// 5ms
// Beats62.16%
// Analyze Complexity
// Memory
// 2.15MB
// Beats35.14%


impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        
        let mut vi = vec![0; word.len()];
        let mut t2 = 0;
        for i in (0..word.len()).rev() {
            vi[i] = t2;
            if "aeiou".contains(word.chars().nth(i).unwrap()) {
                t2 += 1;
            } else {
                t2 = 0;
            }
        }

        let mut typecnt = 0;
        let mut cnt = 0;
        let mut st = 0usize;
        let mut arr = [0i32; 5];
        let mut ans = 0i32;
        for (idx, ch) in word.chars().enumerate() {

            let i = match ch {
                'a' => 0usize,
                'e' => 1usize,
                'i' => 2usize,
                'o' => 3usize,
                'u' => 4usize,
                _ => 555usize,
            };

            if i < arr.len() {
                if arr[i] == 0 {
                    typecnt += 1;
                }
                arr[i] += 1;
                cnt += 1;
            }


            while (idx - st + 1 - cnt) as i32 >= k && typecnt == 5 {

                if (idx - st + 1 - cnt) as i32 == k {
                    ans += vi[idx] + 1;
                }

                let i2 = match word.chars().nth(st).unwrap() {
                    'a' => 0usize,
                    'e' => 1usize,
                    'i' => 2usize,
                    'o' => 3usize,
                    'u' => 4usize,
                    _ => 555usize,
                };

                if i2 < arr.len() {
                    cnt -= 1;
                    arr[i2] -= 1;
                    if arr[i2] == 0 {
                        typecnt -= 1;
                    }
                }

                st += 1;
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


