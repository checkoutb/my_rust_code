



// Runtime16 ms         // not print vi
// Beats
// 84.62%
// Memory3.9 MB
// Beats
// 38.46%

// Runtime49 ms
// Beats
// 7.69%
// Memory3.9 MB
// Beats
// 53.85%

// i64 overflow .....

// ? opposite direction == don't change direction ?  so just move d step?
impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let mut vi: Vec<i64> = vec![0; nums.len()];
        let s = s.into_bytes();
        let d = d as i64;
        for i in 0..nums.len() {
            if s[i] == b'L' {
                vi[i] = nums[i] as i64 - d;
            } else {
                vi[i] = nums[i] as i64 + d;
            }
        }
        vi.sort();

        // println!("{:?}", vi);

        const MOD: i64 = 1000000007;
        // let t2: i64 = 1;
        let mut ans: i64 = 0;
        for i in 1..vi.len() {
            // ans += ans + (i as i64) * (vi[i] - vi[i - 1]);
            ans += (i as i64) * ((vi.len() - i) as i64) % MOD * (vi[i] - vi[i - 1]) % MOD;
            ans %= MOD;
        }
        (ans % MOD) as i32
    }
}

// -3, -1, 1


struct Solution {}

fn main()
{

    // let vi = [-2,0,2].to_vec();
    // let s = "RLL".to_string();
    // let d = 3;

    // let vi = [1,0].to_vec();
    // let s = "RL".to_string();
    // let d = 2;

    let vi = [-10,-13,10,14,11].to_vec();       // 146
    let s = "LRLLR".to_string();
    let d = 2;
    // -11, -12, 9, 13, 12
    // -12, -11, 8, 12, 13
/*
1-0, 2-0, 3-0, 4-0
    2-1, 3-1, 4-1
        3-2, 4-2
            4-3

1-0, 2-1, 1-0, 3-2, 2-1, 1-0, 4-3, 3-2, 2-1, 1-0
2-1, 3-2, 2-1, 4-3, 3-2, 2-1
3-2, 4-3, 3-2
4-3

1-0     4       1*4
2-1     3+3     2*3
3-2     2+2+2   3*2
4-3     1+1+1+1 4*1

*/

    println!("ans: {:?}", Solution::sum_distance(vi, s, d));

    let vi = [-12, -11, 8, 12, 13].to_vec();
    let mut t2 = 0;
    for i in 0..vi.len() {
        for j in (i + 1)..vi.len() {
            t2 += vi[j] - vi[i];
        }
    }
    println!("asd {}", t2);
}


