

// D

// unordered_map<double, int> cnt;
// for (int r = 4; r < n - 2; ++r) {
//     int q = r - 2;
//     for (int p = 0; p < q - 1; ++p)
//         cnt[1.0 * A[p] / A[q]]++;
//     for (int s = r + 2; s < n; ++s)
//         res += cnt[1.0 * A[s] / A[r]];
// }

// wodefa.


// hint:  0/1 == 3/2 ... gcd  
// 但是 还是不好写。

// g


// [0]*[2] == [1]*[3]
// idx 0<1<2<3
// idx 不相连

// 长度1000，值1000

// ?  0/1 == 3/2  但是会 小数啊。

// ok, vvi, 保存 值出现的下标， 然后 遍历值， 如果 值出现了4次，那么必然有一个 subseq，不 还要 间隙大于1.
// 2个值，如果 都有2次出现，那么可以 尝试配对

// 感觉需要 range query，就是 p q r s， 从后往前遍历， 对于每个 p s 对，要 range query 这个范围内的 具有相同值的下标(且有间隙)的对数。
// 1000 能硬算吧。
// 不过 下标 必须有 间隙， 不知道好不好处理。
// ..感觉会tle啊。

// DP!... 想到一个。  就像 反向 扎气球，  两侧增加2个数，如果这2个数 相同，那么 里面的 所有 pair 都可以组成一个 subseq
// 间隙也好处理， 再往里面一层就可以了。
// g, 错的， 13 24 不是  14 23。。。  2侧新增的数 可以不同。

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let sz1 = nums.len();
        let mut vvi = vec![vec![0; sz1]; sz1];   // [i, j] 中有多少 相同的pair，且 有间隙
        let mut sublen = 2;   // 1 是没有间隙，不能用的。
        let mut ans = 0;
        while sublen < sz1 {

            for i in 0..sz1 {
                let j = i + sublen;
                if j >= sz1 {
                    break;
                }

                if nums[i] != nums[j] {
                    vvi[i][j] = vvi[i + 1][j - 1].max(vvi[i + 1][j].max(vvi[i][j - 1]));
                } else {
                    vvi[i][j] = vvi[i + 1][j - 1] + 1;
                    vvi[i][j] = vvi[i][j].max(vvi[i + 1][j].max(vvi[i][j - 1]));
                    // subseq
                    ans += vvi[i + 2][j - 2]; // ?
                    println!("{} {} {}", ans, i, j);
                }
            }
            sublen += 1;
        }

        // println!("{:?}", vvi);
        for i in 0..vvi.len() {
            println!("{:?}", vvi[i]);
        }

        ans
    }
}


// impl Solution {
//     pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
//         let mut mxval = nums[0];
//         let mut vvi = vec![vec![]; 1001];
//         for i in 0..nums.len() {
//             if nums[i] > mxval {
//                 mxval = nums[i];
//             }
//             vvi[nums[i]].push(i);
//         }
        
//         for i in 0..vvi.len() {
//             if vvi.len() <= 1 {
//                 continue;
//             }
//             if i as i32 > mxval {
//                 break;
//             }

//             if vvi.len() >= 4 {

//             }
//         }

//     }
// }


struct Solution {}

fn main()
{

    let vi = [3,4,3,4,3,4,3,4].to_vec();


    println!("ans: {:?}", Solution::number_of_subsequences(vi));
}


