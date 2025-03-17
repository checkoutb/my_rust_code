

// D D

// vector<vector<int>> ncr(nums.size() + 1, vector<int> (k + 1,0));
        
// ncr[0][0] = 1;
// for (int n = 1; n <= nums.size(); n++) {
//     ncr[n][0] = 1;
//     for (int r = 1; r <= k; r++) 
//         ncr[n][r] = (ncr[n - 1][r - 1] + ncr[n - 1][r]) % MOD;
// } 
// ..................................



// Runtime
// 127ms
// Beats78.26%
// Memory
// 3.96MB
// Beats82.61%




// subseq 的长度 <= k

// ???  大数 nCr ? 但是它需要质数。
// 现在的想法是： sort，  从后往前，  以当前为最小值， 后面取 0,1,2,3,4,... 个元素，组成 subseq。
//    从前往后， 当前是最大值，前面 取 xxx 个元素。

// nCm    n!/(m! * (n-m)!)    .. 除法，没办法 MOD啊。

// 5C1  5!/1!/4!   5C2  5!/2!/3!
// 5!/4!/1!  5!/3!/2!

// 70!  100个0 ..

// 因数分解。

// 那就硬算啊。    不不不，， 能dp？ 似乎可以？


impl Solution {
    pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
        
        let mut nums = nums;
        nums.sort();
        const MOD: i64 = 1000000007;
        let mut ans = 0;
        let mut vi = vec![0i64; k as usize];
        for i in 0..nums.len() {
            // let mut v2 = vec![0; k as usize];
            let mut v2 = vi.clone();

            for j in 0..(k - 1) as usize {   // [i] is max elements
                let t3: i64 = vi[j] * (1 + 0 as i64) % MOD;
                v2[j + 1] += t3;    // 0->1, 1->2
                ans = (ans + t3 * nums[i] as i64) % MOD;

                // println!("{}, {}, {}", ans, t3, nums[i]);
                // println!("           {}", vi[j]);

            }
            v2[0] += 1;
            ans = (ans + nums[i] as i64) % MOD;
            vi = v2;

            // println!("{}", ans);
            // println!(" > {:?}", vi);
        }

        vi = vec![0i64; k as usize];
        for i in (0..nums.len()).rev() {
            let mut v2 = vi.clone();

            for j in 0..(k - 1) as usize {
                let t3 : i64 = vi[j] * (1 + 0 as i64) % MOD;
                v2[j + 1] += t3;
                ans = (ans + t3 * nums[i] as i64) % MOD;

                // println!("{}, {}, {}", ans, t3, nums[i]);

            }
            v2[0] += 1;
            ans = (ans + nums[i] as i64) % MOD;

            vi = v2;
            // println!("   {}", ans);
        }

        ans as i32
    }
}

struct Solution {}

fn main()
{

    // let vi = [1,2,3].to_vec();
    // let k = 2;

    // 0, 1, 1     1 1 1 1
    // 01, 01, 11   1 1 1 1
    // 011        1

    let vi = [0,1,1].to_vec();
    let k = 3;

    println!("ans: {:?}", Solution::min_max_sums(vi, k));
}


