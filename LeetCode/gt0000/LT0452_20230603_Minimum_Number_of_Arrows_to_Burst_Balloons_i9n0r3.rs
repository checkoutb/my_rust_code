





// Runtime46 ms
// Beats
// 94.12%
// Memory9.7 MB
// Beats
// 5.88%

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        
        let mut vvi = points;
        vvi.sort_by(|a, b| a[1].cmp(&b[1]));

        //println!("{}", vvi);
        for i in 0..vvi.len() {
            // println!("{}", vvi[i]);
            for j in 0..vvi[i].len() {
                println!("{}, ", vvi[i][j]);
            }
        }

        let mut ans: i32 = 0;
        //let mut shot: i32 = i32::MIN;
        let mut shot: i32 = vvi[0][1];
        ans += 1;
        for (_i, &ref vi) in vvi.iter().enumerate() {
            if vi[0] > shot {
                //ans++;        // not valid .. ? i++ ?
                ans += 1;
                shot = vi[1];
            }
        }

        ans
    }
}




struct Solution {}

fn main()
{

    let vvi = vec![vec![10,16], vec![2,8],vec![1,6],vec![7,12]];

    println!("ans: {:?}", Solution::find_min_arrow_shots(vvi));
}


