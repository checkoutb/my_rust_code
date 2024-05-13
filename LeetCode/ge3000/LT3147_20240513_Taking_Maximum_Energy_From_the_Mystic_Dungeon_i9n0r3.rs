

// for(int j = n - k - 1; j >= 0; j--){
//     energy[j] += energy[j + k];
// }
// for(int i = 0; i < n; i++){
//     ans = max(ans, energy[i]);
// }


// vector<int> res(k, -1000);
// for (int i = 0; i < e.size(); ++i)
//     res[i % k] = max(e[i], res[i % k] + e[i]);
// return *max_element(begin(res), end(res));


// Runtime
// 25ms
// Beats100.00%of users with Rust
// Memory
// 2.90MB
// Beats100.00%of users with Rust

// kadane
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut ans = -12345;

        for i in 0..k {
            let mut t2 = 0;
            let mut j = i as usize;
            while j < energy.len() {
                t2 = t2.max(0);     // 
                t2 += energy[j];
                j += k as usize;
            }
            ans = ans.max(t2);
        }
        ans
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


