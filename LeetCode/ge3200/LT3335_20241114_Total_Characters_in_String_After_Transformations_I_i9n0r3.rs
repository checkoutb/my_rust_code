







// Runtime
// 26ms
// Beats63.49%
// Analyze Complexity
// Memory
// 2.33MB
// Beats88.89%


impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 1000000007;
        let mut vi = [0i32; 26];
        s.into_bytes().into_iter().for_each(|e| vi[(e - b'a') as usize] += 1);

        for _i in 0..t as usize {
            let mut v2 = [0i32; 26];

            for i in 0..25 {
                v2[i + 1] = vi[i];
            }
            v2[0] = vi[25];
            v2[1] = (v2[1] + vi[25]) % MOD;

            vi = v2;
        }
        let mut ans = 0i32;
        vi.into_iter().for_each(|e| ans = (ans + e) % MOD);
        ans
    }
}



struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


