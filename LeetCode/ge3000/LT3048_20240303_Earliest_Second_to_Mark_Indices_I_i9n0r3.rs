

// D D

// int l = 0, r = m + 1;
// while (l < r) {
//     int mid = l + (r - l) / 2;
//     if (binarySearch(nums, changeIndices, mid)) {
//         r = mid;
//     } else {
//         l = mid + 1;
//     }
// }
// 

// bool binarySearch(vector<int>& nums, vector<int>& changeIndices, int idx) {
//     unordered_map<int, int> last;
//     for (int i = 0; i < idx; i++) {
//         last[changeIndices[i]] = i;
//     }
//     if (last.size() != nums.size()) return false;
//     int cnt = 0; // record we can reduce how many number
//     for (int i = 0; i < idx; i++) {
//         // if it is last time we visit this idx, we must mark
//         // so check whether this idx already reduce to zero, if not, then return false.
//         if (i == last[changeIndices[i]]) {
//             if (cnt < nums[changeIndices[i] - 1]) return false;
//             else cnt -= nums[changeIndices[i] - 1];
//         } else {
//             cnt++;
//         }
//     }
//     return true;
// }




// 0ms 给我希望，看了下II，结果题目有改动，并不是单纯的Constraints的修改。

// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.22MB
// Beats14.63%of users with Rust


// Runtime
// 0ms
// Beats100.00%of users with Rust
// Memory
// 2.18MB
// Beats65.85%of users with Rust


// decre farest ( max index that not 0), no... range [0, n]  not [0, changeindex[s]]
// wait the changeidx[s]
// 
// ??? only binary search ?
impl Solution {
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let sz1 = nums.len();
        let sz2 = change_indices.len();
        let mut st = 1usize;
        let mut en = sz2;
        let mut ans = sz2 + 4;
        let sum2 = nums.iter().map(|i| *i as i64).sum::<i64>();
        let mut vi = vec![sz2 + 2; sz1];
        let mut nxt = vec![sz2 + 2; sz2];
        for a in 0..sz2 {
            let i = sz2 - 1 - a;
            nxt[i] = vi[change_indices[i] as usize - 1];
            vi[change_indices[i] as usize - 1] = i;
        }
        while st <= en {
            let md = (st + en) / 2;
            if (md as i64) < sum2 + (sz1 as i64) {
                st = md + 1;
                continue;
            }
            let mut can = true;
            // let mut vb = vec![false; sz1 + 1];
            let mut cnt = 0;
            let mut cnt2 = 0usize;
            for i in 0..md {
                if nxt[i] >= md {
                    cnt -= nums[change_indices[i] as usize - 1];
                    if cnt < 0
                    {
                        can = false;
                        break;
                    }
                    cnt2 += 1;
                } else {
                    cnt += 1;
                }
            }
            if can && cnt2 == sz1 {
                ans = md;
                en = md - 1;
            } else {
                st = md + 1;
            }
        }
        if ans == sz2 + 4 {
            -1
        } else {
            ans as i32
        }

        // let sz1 = nums.size();
        // let mut vb = vec![false; sz1 + 1];      // [1, n]
        // let mut cnt = sz1;
        // for i in 0..change_indices.len() {
        //     if vb[change_indices[i] as usize] == false {
        //         vb[change_indices[i] as usize] = true;
        //         --cnt;
        //     }
        // }
    }
}


struct Solution {}

fn main()
{

    // let vi = [2,2,0].to_vec();
    // let v2 = [2,2,2,2,3,2,2,2,2,2,2,1].to_vec();

    let vi = [1,3].to_vec();
    let v2 = [1,1,1,2,1,1,1,1].to_vec();

    println!("ans: {:?}", Solution::earliest_second_to_mark_indices(vi, v2));
}


