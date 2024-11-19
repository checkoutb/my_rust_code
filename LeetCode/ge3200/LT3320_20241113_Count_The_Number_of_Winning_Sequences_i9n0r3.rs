



// D D

// long long dp[2][2001][3] = {};
// dp[0][1000][0] = 1;
// for (int i = 0; i < sz; ++i) {
//     int a = s[i] == 'F' ? 2 : s[i] == 'E' ? 1 : 0;
//     for (int b = 0; b < 3; ++b) {
//         // F > E, E > W, W > F
//         int pt = a == b ? 0 : a == 0 && b == 2 ? -1 : b == 0 && a == 2 ? 1 : a > b ? -1 : 1;
//         for (int j = 1000 - i; j < 1001 + i; ++j)
//             dp[!(i % 2)][j + pt][b] = (
//                 dp[i % 2][j][0] + dp[i % 2][j][1] + dp[i % 2][j][2] - (i == 0 ? 0 : dp[i % 2][j][b])
//             ) % mod;
//     }
// }



// Runtime
// 271ms
// Beats50.00%
// Analyze Complexity
// Memory
// 2.31MB
// Beats69.44%



// .... hashmap... 有点难用。 不过写完后，看起来很不错 ( 指 entry，into_iter)。


// F > E > W > F
// bob no continue 2
// record: bob win count - alice win count, bob last summon
// s.len < 1000
// 1000 * 2000 * 3
// array or map?
impl Solution {
    pub fn count_winning_sequences(s: String) -> i32 {
        use std::collections::HashMap;

        let mut mpf: HashMap<i32, i32> = HashMap::new();  // bob last summon is f
        let mut mpe: HashMap<i32, i32> = HashMap::new();  // <bob win count - alice win count, count>
        let mut mpw: HashMap<i32, i32> = HashMap::new();

        const MOD: i32 = 10i32.pow(9) + 7;

        let s = s.as_bytes();

        match s[0] {
            b'F' => {
                // mpf[0] = 0; //.....
                mpf.insert(0, 1);
                mpe.insert(-1, 1);
                mpw.insert(1, 1);
            },
            b'E' => {
                mpf.insert(1, 1);
                mpe.insert(0, 1);
                mpw.insert(-1, 1);
            }
            _ => {  // b'W'
                mpf.insert(-1, 1);
                mpe.insert(1, 1);
                mpw.insert(0, 1);
            }
        }

        for i in 1..s.len() {
            let mut mp2f: HashMap<i32, i32> = HashMap::new();
            let mut mp2e: HashMap<i32, i32> = HashMap::new();
            let mut mp2w: HashMap<i32, i32> = HashMap::new();

            for (k, v) in mpf.into_iter() {
                // no f
                match s[i] {
                    b'F' => {
                        mp2e.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    b'E' => {
                        mp2e.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    _ => {
                        mp2e.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    }
                }
            }
            for (k, v) in mpe.into_iter() {
                // no e
                match s[i] {
                    b'F' => {
                        mp2f.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    b'E' => {
                        mp2f.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    _ => {
                        mp2f.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2w.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    }
                }
            }
            // F > E > W > F
            for (k, v) in mpw.into_iter() {
                // no w
                match s[i] {
                    b'F' => {
                        mp2f.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2e.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    b'E' => {
                        mp2f.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2e.entry(k).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    },
                    _ => {
                        mp2f.entry(k - 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                        mp2e.entry(k + 1).and_modify(|e| *e = (v + *e) % MOD).or_insert(v);
                    }
                }
            }
            mpf = mp2f;
            mpe = mp2e;
            mpw = mp2w;
        }

        let mut ans = 0;
        mpf.into_iter().for_each(|(k, v)| if k > 0 { ans = (ans + v) % MOD } );
        mpe.into_iter().for_each(|(k, v)| if k > 0 { ans = (ans + v) % MOD } );
        mpw.into_iter().for_each(|(k, v)| if k > 0 { ans = (ans + v) % MOD } );

        ans
    }
}



struct Solution {}

fn main()
{

    // let s = "FFF".to_string();
    let s = "FWEFW".to_string();

    println!("ans: {:?}", Solution::count_winning_sequences(s));
}


