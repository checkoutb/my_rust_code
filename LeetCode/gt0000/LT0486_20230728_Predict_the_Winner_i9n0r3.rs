



// Runtime2 ms
// Beats
// 75%
// Memory2 MB
// Beats
// 58.33%

// feichang jingdian..

// sz < 20
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let sz1 = nums.len();
        let mut vvi = vec![vec![0; sz1]; sz1];
        let mut vi = vec![nums[0]; sz1];
        for i in 1..sz1 {
            vi[i] = nums[i] + vi[i - 1];
        }

        for il in 1..=sz1 {
            for j in 0..=(sz1 - il) {
                // vvi[j][j + il - 1] = (il == 1 ? nums[j] : 
                //     (nums[j] + (j == 0 ? vi[j + il - 1] : (vi[j + il - 1] - vi[j - 1])) - vvi[j + 1][j + il - 1])
                //     .max(nums[j + il - 1] - vvi[j + 1][j + il - 2] + (vi[j + il - 1] - (j == 0 ? 0 : vi[j - 1]))));
                
                if il == 1 {
                    vvi[j][j + il - 1] = nums[j];
                } else {
                    let t2 = if j == 0 {vi[j + il - 1]} else {vi[j + il - 1] - vi[j - 1]};
                    vvi[j][j + il - 1] = (t2 - vvi[j + 1][j + il - 1]).max(t2 - vvi[j][j + il - 2]);
                }
            }
        }
        // println!("{:?}", vvi);
        println!("{:?}", vi);
        for i in 0..sz1 {
            println!("{:?}", vvi[i]);
        }
        return vvi[0][sz1 - 1] >= (vi[sz1 - 1] - vvi[0][sz1 - 1]);
    }
}



struct Solution {}

fn main()
{
    // let vi = [1,5,2233,3].to_vec();
    let vi = [1,5,2,4,6].to_vec();  // true   // 6 2 1

    println!("ans: {:?}", Solution::predict_the_winner(vi));
}


