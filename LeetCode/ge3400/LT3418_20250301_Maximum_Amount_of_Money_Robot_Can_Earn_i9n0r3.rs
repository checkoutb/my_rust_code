


// Runtime
// 116ms
// Beats43.48%
// Memory
// 16.78MB
// Beats52.17%




// 可以为负， 看成不可以为负了。


impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let sz1 = coins.len();
        let sz2 = coins[0].len();

        let mut vvvi = vec![vec![vec![i32::MIN; 3]; sz2]; sz1];  // 避免 0/1/2 次

        vvvi[0][0][0] = coins[0][0];
        if coins[0][0] < 0 {
            vvvi[0][0][1] = 0;
        }

        for i in 0..sz1 {
            for j in 0..sz2 {
                if coins[i][j] >= 0 {
                    for k in 0..3 {   // 这里 和 下面的 be robbed 一样的。
                        if i > 0 && vvvi[i - 1][j][k] != i32::MIN {
                            vvvi[i][j][k] = (vvvi[i - 1][j][k] + coins[i][j]).max(vvvi[i][j][k]);
                        }
                        if j > 0 && vvvi[i][j - 1][k] != i32::MIN {
                            vvvi[i][j][k] = (vvvi[i][j - 1][k] + coins[i][j]).max(vvvi[i][j][k]);
                        }
                    }
                } else {
                    for k in 0..2 {  // neutralize
                        if i > 0 && vvvi[i - 1][j][k] != i32::MIN {
                            vvvi[i][j][k + 1] = (vvvi[i - 1][j][k]).max(vvvi[i][j][k + 1]);
                        }
                        if j > 0 && vvvi[i][j - 1][k] != i32::MIN {
                            vvvi[i][j][k + 1] = (vvvi[i][j - 1][k]).max(vvvi[i][j][k + 1]);
                        }
                    }
                    for k in 0..3 { // be robbed
                        if i > 0 && vvvi[i - 1][j][k] != i32::MIN {
                            // if vvvi[i - 1][j][k] + coins[i][j] >= 0 {
                                vvvi[i][j][k] = vvvi[i][j][k].max(vvvi[i - 1][j][k] + coins[i][j]);
                            // }
                        }
                        if j > 0 && vvvi[i][j - 1][k] != i32::MIN {
                            // if vvvi[i][j - 1][k] + coins[i][j] > vvvi[i][j][k] {
                                // vvvi[i][j][k] = vvvi[i][j - 1][k] + coins[i][j];
                            // }
                            vvvi[i][j][k] = vvvi[i][j][k].max(vvvi[i][j - 1][k] + coins[i][j]);
                        }
                    }
                }
            }
        }

        vvvi[sz1 - 1][sz2 - 1][0].max(vvvi[sz1 - 1][sz2 - 1][1].max(vvvi[sz1 - 1][sz2 - 1][2]))
    }
}



struct Solution {}

fn main()
{

    // let vvi = [[0,1,-1].to_vec(),[1,-2,3].to_vec(),[2,-3,4].to_vec()].to_vec();

    let vvi = [
        [-3,-10,11,-16].to_vec(),
        [-13,19,-2,2].to_vec(),
        [-16,-11,5,13].to_vec(),
        [-5,13,-20,-6].to_vec()].to_vec();

    println!("ans: {:?}", Solution::maximum_amount(vvi));
}


