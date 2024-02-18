




// D

// KMP
// rabin-karp


// g






impl Solution {
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut vi = Vec::new();
        vi.resize(nums.len() - 1, 0);

        // println!("{}", vi.len());

        for i in 0..(nums.len() - 1) {
            if nums[i] < nums[i + 1] {
                vi[i] = 1;
            } else if nums[i] > nums[i + 1] {
                vi[i] = -1;
            }
        }
        let mut pfx1 = vec![(0,0,0); pattern.len()];        // -1, 0, 1 's count
        let mut pfx2 = vec![(0,0,0); vi.len()];
        
        // Self::adda1(&mut pfx1[0], &mut pfx1[0], pattern[0]);
        Self::adda1(&mut pfx1, 0, pattern[0]);
        for i in 1..pattern.len() {
            // Self::adda1(&mut pfx1[i], &mut pfx1[i - 1], pattern[i]);
            Self::adda1(&mut pfx1, i, pattern[i]);
        }

        // Self::adda1(&mut pfx2[0], &mut pfx2[0], vi[0]);
        Self::adda1(&mut pfx2, 0, vi[0]);
        for i in 1..vi.len() {
            // Self::adda1(&mut pfx2[i], &mut pfx2[i - 1], vi[i]);
            Self::adda1(&mut pfx2, i, vi[1]);
        }

        println!("{:?}\n{:?}", pfx1, pfx2);
        let mut ans = 0;

        for i in 0..(nums.len() - pattern.len()) {
            if match_pattern() {
                ans += 1;
            }
        }

        ans
    }

    fn match_pattern(pfx1: &Vec<(i32, i32, i32)>, pfx2: &Vec<(i32, i32, i32)>, idx: usize) -> bool {
        let en = idx + pfx1.len();

    }

    fn adda1(pfx: &mut Vec<(i32, i32, i32)>, idx: usize, val: i32) {
        if idx > 0 {
            pfx[idx].0 = pfx[idx - 1].0;
            pfx[idx].1 = pfx[idx - 1].1;
            pfx[idx].2 = pfx[idx - 1].2;
        }
        if val == -1 {
            pfx[idx].0 += 1;
        } else if val == 0 {
            pfx[idx].1 += 1;
        } else {
            pfx[idx].2 += 1;
        }
    }

    // fn adda1(tup : &mut(i32, i32, i32), pfxtup: &mut(i32, i32, i32), val: i32) {
    //     tup.0 = pfxtup.0;
    //     tup.1 = pfxtup.1;
    //     tup.2 = pfxtup.2;
    //     if val == -1 {
    //         tup.0 += 1;
    //     } else if val == 0 {
    //         tup.1 += 1;
    //     } else {
    //         tup.2 += 1;
    //     }
    // }
}


struct Solution {}

fn main()
{

    let vi = [1,2,3,4,5,6].to_vec();
    let v2 = [1,1].to_vec();

    println!("ans: {:?}", Solution::count_matching_subarrays(vi, v2));
}


