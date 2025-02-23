



// Runtime
// 12ms
// Beats11.11%
// Memory
// 10.26MB
// Beats88.89%

// 找到了 1463 cherry pickup II， 2个机器人采樱桃  
// 72% 通过率，但是。。


/*

2个机器人采苹果。  但是忘记了。

00   0n-1

n-10

count of fruit

n-1 move  reach n-1n-1

00:  i+1,j  i,j+1,  i+1,j+1      右，下，  右下

0n-1: i+1,j-1, i+1,j, i+1,j+1    左下，下，  右下

n-10: i-1,j+1, i,j+1, i+1,j+1    右上，右，  右下

1000*1000

n-1步到达 n-1n-1， 00的话 必须一直 右下啊。

0n-1 最多 45度 左下 走一半，然后 45度 右下 走到 n-1n-1

n-10  45度 右上， 走一半， 45度右下。

所以变成 2个人了。 
而且。 0n-1  和 n-10 是不会 相交的， 一旦相交，就没有办法 走到 n-1n-1了

所以 不需要 考虑 另外一个人， 只需要考虑自己： 获得最多水果 且 n-1步能到达 n-1n-1

0n-1 可以和 00相交。 但走上去没有意义。 有意义的 [sz1-1][sz1-1]..

*/

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut fruits = fruits;

        for i in 0..fruits.len() {
            ans += fruits[i][i];
            fruits[i][i] = 0;
        }

        // for i in 0..fruits.len() {
        //     let mut j = fruits.len() - 1;   // [0][n-1]
        //     while i + j < fruits.len() && j < fruits.len() {


        //         j -= 1;
        //     }
        // }

        // let sz1 = fruits.len();
        // fruits[0][sz1 - 1] = -fruits[0][sz1 - 1] - 1;

        let sz1 = fruits.len();
        let mut vvb = vec![vec![false; sz1]; sz1];
        vvb[0][sz1 - 1] = true;

        for i in 1..sz1 {
            let mut j = sz1 - 1;
            while j >= 0 && j >= i {
                let mut t2 = 0;
                if j + 1 < sz1 && !vvb[i - 1][j + 1] {
                    break;
                } else {
                    if j + 1 < sz1 {
                        t2 = fruits[i - 1][j + 1];
                    }
                }
                if vvb[i - 1][j] {
                    t2 = t2.max(fruits[i - 1][j]);
                }
                if vvb[i - 1][j - 1] {
                    t2 = t2.max(fruits[i - 1][j - 1]);
                }
                vvb[i][j] = true;
                fruits[i][j] += t2;
                j -= 1;
            }
        }

        for i in 0..sz1 - 1 {
            fruits[i][i] = 0;
            vvb[i][i] = false;
        }

        vvb[sz1 - 1][0] = true;
        for j in 1..sz1 {
            let mut i = sz1 - 1;
            while i >= j {
                let mut t2 = 0;
                if i + 1 < sz1 && !vvb[i + 1][j - 1] {
                    break;
                } else {
                    if i + 1 < sz1 {
                        t2 = fruits[i + 1][j - 1];
                    }
                }
                if vvb[i][j - 1] {
                    t2 = t2.max(fruits[i][j - 1]);
                }
                if vvb[i - 1][j - 1] {
                    t2 = t2.max(fruits[i - 1][j - 1]);
                }
                vvb[i][j] = true;
                fruits[i][j] += t2;
                i -= 1;
            }
        }

        // println!("ans {}", ans);
        // println!("{:?}", fruits);

        ans + fruits[sz1 - 1][sz1 - 1]
    }
}




struct Solution {}

fn main()
{

    let vvi = [[1,2,3,4].to_vec(), [5,6,8,7].to_vec(), [9,10,11,12].to_vec(), [13,14,15,16].to_vec()].to_vec();

    println!("ans: {:?}", Solution::max_collected_fruits(vvi));
}


