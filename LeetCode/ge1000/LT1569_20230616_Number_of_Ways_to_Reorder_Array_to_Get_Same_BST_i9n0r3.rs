




// Runtime127 ms
// Beats
// 100%
// Sorry, there are not enough accepted submissions to show data
// Memory14.2 MB
// Beats
// 100%


// C6-3 vs. for{C3-x * C3-y}

// 6!/3!/3!  = 20

// 0 - 3!/0!/3! * 3!/1!/2!  1*3
// 1 - 3!/1!/2! * 3!/2!/1!  3*3
// 2 - 3!/2!/1! * 3!/3!/0!  3*1     15 ... != 20....
//      no. not C3-x  C3-y,   

// it is C6-3 vs for {C2-x * C4-y}

// 0 - 2!/0!/2! * 4!/1!/3!  1 * 4
// 1 - 2 * 4!/2/2   2 * 6  
// 2 - 1 * 4                20 == 20



// hint: order x elements in x+y positions?
// ...........................

// g


// left child, right child.   those 2 node's order can swap.   
// left child tree's all node's order can not change.
// l l r r l l l l r r ...
// 2 subsequence alternate, how may way?
// [1,3,5,] , [2,4,6]
//      [1,2,3,4,5,6], [1,3,2,4,6,5] ...
//          dp ?????? no .
//          treat [2,4,6] as one item, so there is 4 site (before 1/3/5, after 5) to insert  1 * 4
//          as 2 item, so [2]+[4,6] or [2,4],[6],  C4-2 == ??? = 4! / 2! / (4-2)! = 6,  2*6
//              C1000-500 ???  dp :  Cm-n = C(m-1)-(n-1) + C(m-1)-n
// 
// ? dp && dp ?    dp1: Cn-m, dp2: [left child cnt][right child cnt] ?
//          {[2]+[4,6] === C2-1.  2(split)4(split)6, 2 choose 1} * { ?1?3?5? , 4?, choose 2}
//              i believe, {[2,4,6]} * {[1,3,7]} == {[1,3,7]}*{[2,4,6]}
impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mut cntl = 0;
        let mut cntr = 0;
        for i in 1..nums.len() {        // maybe there are some filter, map ?
            if nums[i] > nums[0] {
                cntr += 1;
            } else {
                cntl += 1;
            }
        }
        let sz1 = cntl.max(cntr);     // dp's sz1  Cn-m 's n
        let sz2 = sz1 / 2 + sz1 % 2;        // Cn-m 's m
        // let mut cb = vec![vec![0; sz2 + 1]; sz1 + 2];   // combination
        let mut cb = vec![vec![0; nums.len() / 2 + 1]; nums.len() + 1];

        for i in 0..cb.len() {
            cb[i][0] = 1;
        }
        // for i in 0..cb[0].len() {
        //     cb[0][]
        // }
        
        // ur left is not my left
        // let mut vvi = vec![vec![0; cntr + 1]; cntl + 1];    // [left cnt][right cnt]    // map is better
        let mut vvi = vec![vec![0; sz1 + 1]; sz1 + 1];
        // let mut vvi = 

        // ??? to build BST ???  nums.len < 1000, so no tle.
        // ++, top to down ... parent to child...
        let mut vv2 = vec![vec![-1; 2]; nums.len()];     // 0: left, 1: right
        for i in 1..nums.len() {
            Self::inserta1(&mut vv2, &nums, 0, i);
        }

        for vn in &vv2 {
            println!("{:?}", vn);
        }

        let ans = Self::dfsa3(&mut vv2, &mut cb, &mut vvi, 0).0 as i32 - 1;

        for vn in &vvi {
            println!("  {:?}", vn);
        }

        for vn in &cb {
            println!("-- {:?}", vn);
        }

        ans
    }
    
    const MOD: i64 = 1000000007;

    // (ans, cnt)
    fn dfsa3(vv2: &Vec<Vec<i64>>, cb: &mut Vec<Vec<i64>>, vvi: &mut Vec<Vec<i64>>, idx: i64) -> (i64, usize) {
        if idx == -1 {
            return (0, 0);
        }
        let (mut ansl, cntl) = Self::dfsa3(vv2, cb, vvi, vv2[idx as usize][0]);
        let (mut ansr, cntr) = Self::dfsa3(vv2, cb, vvi, vv2[idx as usize][1]);
        if ansl == 0 {
            ansl = 1;
        }
        if ansr == 0 {
            ansr = 1;
        }
        let ans = ansl * ansr % Self::MOD * Self::dfsa2(vvi, cntl, cntr, cb) % Self::MOD;
        println!("{}, {}, {}", idx, ans, cntl + cntr + 1);
        (ans, cntl + cntr + 1)
    }

    fn inserta1(vv2: &mut Vec<Vec<i64>>, nums: &Vec<i32>, idx: usize, i: usize) {
        if nums[i] < nums[idx] {
            if vv2[idx][0] == -1 {
                vv2[idx][0] = i as i64;
            } else {
                Self::inserta1(vv2, nums, vv2[idx][0] as usize, i);
            }
        } else {
            if vv2[idx][1] == -1 {
                vv2[idx][1] = i as i64;
            } else {
                Self::inserta1(vv2, nums, vv2[idx][1] as usize, i);
            }
        }
    }

    // split, merge 2 subsequence
    fn dfsa2(vvi: &mut Vec<Vec<i64>>, szl: usize, szr: usize, cb: &mut Vec<Vec<i64>>) -> i64 {
        if vvi[szl][szr] != 0 {
            return vvi[szl][szr];
        }
        // vvi[szl + szr][szl.min(szr)] = Self::dfsa1(cb, szl + szr, szl);
        // vvi[szl + szr][szl.min(szr)]
        vvi[szl.max(szr)][szl.min(szr)] = Self::dfsa1(cb, szl + szr, szl);
        vvi[szl.max(szr)][szl.min(szr)]

        // if szl == 0 || szr == 0 {
        //     return 1;
        // }
        // let mut t2 = 0;
        // for i in 0..szl {       // split cnt.  +1 == part cnt
        //     t2 += Self::dfsa1(cb, szl - 1, i) * Self::dfsa1(cb, szr + 1, i + 1);
        //     t2 %= Self::MOD;
        // }
        // vvi[szl][szr] = t2;
        // t2
    }

    // combination
    fn dfsa1(cb: &mut Vec<Vec<i64>>, n: usize, m: usize) -> i64 {
        if n == 0 || n < m {
            return 1;
        }
        let mut m = m;
        if m > n - m {
            m = n - m;
        }
        if m == 0 {
            return 1;
        }
        println!("? {}, {}", n, m);
        if cb[n][m] != 0 {
            return cb[n][m];
        }
        cb[n][m] = (Self::dfsa1(cb, n - 1, m) + Self::dfsa1(cb, n - 1, m - 1)) % Self::MOD;

        println!("cb : {}, {}, {}", n, m, cb[n][m]);

        cb[n][m]
    }

}



struct Solution {}

fn main()
{
    // let nums = [3,4,5,1,2].to_vec();
    // let nums = [2,1,3].to_vec();
    // let nums = [1,2,3].to_vec();
    // let nums = [5,1,4,2,3].to_vec();
    // let nums = [12,13,4,2,9,5,11,3,10,7,1,15,14,8,6].to_vec();
    let nums = [12,13,4,2,9,5,11,3,10,7,1,15,14,8,6].to_vec();  // 2620799

    println!("ans: {:?}", Solution::num_of_ways(nums));
}


