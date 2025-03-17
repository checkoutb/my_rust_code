







// Runtime
// 6ms
// Beats66.67%
// Memory
// 2.52MB
// Beats33.33%


// tle. +vva, +vvb.  AC.   有差这么多吗？


// 1-2M   M=max(M,X)  M=1
// first X

// 100. dfs


impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {

        let mut vva = vec![vec![(-1, -1); 101]; 101];
        let mut vvb = vec![vec![(-1, -1); 101]; 101];
        let ans = Self::dfsa1(&piles, 0, true, 1, &mut vva, &mut vvb).0;
        ans
    }


    // (alice, bob)
    fn dfsa1(piles: &Vec<i32>, idx: usize, is_alice: bool, m: i32, vva: &mut Vec<Vec<(i32, i32)>>, vvb: &mut Vec<Vec<(i32, i32)>>) -> (i32, i32) {
        if idx >= piles.len() {
            return (0, 0);
        }

        if is_alice {
            if vva[idx][m as usize].0 != -1 {
                return vva[idx][m as usize];
            }
        } else {
            if vvb[idx][m as usize].0 != -1 {
                return vvb[idx][m as usize];
            }
        }
        
        let mut gota = 0;
        let mut gotb = 0;
        let mut got_now = 0;
        if is_alice {
            // max gota
            for i in 1..=(2 * m) as usize {
                if idx + i - 1 == piles.len() {
                    break;
                }
                got_now += piles[idx + i - 1];
                let (ga, gb) = Self::dfsa1(piles, idx + i, false, m.max(i as i32), vva, vvb);
                if ga + got_now > gota {
                    gota = ga + got_now;
                    gotb = gb;
                }
            }
        } else {
            // max gotb
            for i in 1..=(2 * m) as usize {
                if idx + i - 1 == piles.len() {
                    break;
                }
                got_now += piles[idx + i - 1];
                let (ga, gb) = Self::dfsa1(piles, idx + i, true, m.max(i as i32), vva, vvb);
                if gb + got_now > gotb {
                    gota = ga;
                    gotb = gb + got_now;
                }
            }
        }
        if is_alice {
            vva[idx][m as usize].0 = gota;
            vva[idx][m as usize].1 = gotb;
        } else {
            vvb[idx][m as usize].0 = gota;
            vvb[idx][m as usize].1 = gotb;
        }
        (gota, gotb)
    }
}


struct Solution {}

fn main()
{

    let vi = [2,7,9,4,4].to_vec();

    println!("ans: {:?}", Solution::stone_game_ii(vi));
}


