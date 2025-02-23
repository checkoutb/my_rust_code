

// D D

// fn compute_cumul(data: &[i32]) -> Vec<i64> {
//     data.iter()
//         .scan(0, |s, v| {
//             *s += *v as i64;
//             Some(*s)
//         })
//         .collect()
// }
// let next_cumul = compute_cumul(&next_cost);
// let prev_cumul = compute_cumul(&previous_cost);





// Runtime
// 6ms
// Beats100.00%
// Memory
// 2.57MB
// Beats100.00%



// a-b-c-...z-a  next cost
// a-z-y-x..a-z  previous cost

// min total cost

// 26 * 26

impl Solution {
    pub fn shift_distance(s: String, t: String, next_cost: Vec<i32>, 
        previous_cost: Vec<i32>) -> i64 {
        
        let mut vvn: Vec<Vec<i64>> = vec![vec![0; 26]; 26];
        let mut vvp: Vec<Vec<i64>> = vec![vec![0; 26]; 26];

        for i in 0..26 {
            let mut stp: i64 = 0;
            for j in i+1..26 {
                stp += next_cost[j - 1] as i64;
                vvn[i][j] = stp;
            }
            for j in 0..i {
                stp += next_cost[(j + 26 - 1) % 26] as i64;     // z->a
                vvn[i][j] = stp;
            }
        }

        for i in 0..26 {
            let mut stp: i64 = 0;
            let mut j = (i + 26 - 1) % 26;
            while j >= 0 && j < i {     // && j < i .............
                stp += previous_cost[(j + 1) % 26] as i64;
                vvp[i][j] = stp;

                // if i == 0 && j == 0 {
                //     println!("+ {}", stp);
                // }


                // j -= 1;   // j is usize... 0usize - 1...
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            j = 25;
            while j > i {
                stp += previous_cost[(j + 1) % 26] as i64;
                vvp[i][j] = stp;
                // j -= 1;

                // if i == 0 && j == 0 {
                //     println!(" - {}", stp);
                // }

                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
        }

        for i in 0..26 {
            for j in 0..26 {

                // println!("{}+{}: {}, {}", i, j, vvn[i][j], vvp[i][j]);

                vvp[i][j] = vvp[i][j].min(vvn[i][j]);
            }
        }

        let mut ans: i64 = 0;
        let au8 = b'a';
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            ans += vvp[(s[i] - au8) as usize][(t[i] - au8) as usize];
        }

        
        ans
    }
}



struct Solution {}

fn main()
{

    let s = "abab".to_string();
    let t = "baba".to_string();

    let n = [100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();
    let p = [1,100,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0].to_vec();

    println!("ans: {:?}", Solution::shift_distance(s,t,n,p));
}


