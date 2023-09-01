


// D D


// let mut max_reach = vec![0; n + 1];
// for i in 0..ranges.len() {
//     let start = i.saturating_sub(ranges[i] as usize);
//     let end = n.min(i + ranges[i] as usize);
//     max_reach[start] = max_reach[start].max(end);
// }
// let (mut taps, mut nxt_end, mut cur_end) = (0, 0, 0);





// Runtime5 ms
// Beats
// 30%
// Memory2.5 MB
// Beats
// 10%

// farest
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut vvi = vec![vec![]; n as usize + 1];
        for i in 0..ranges.len() {
            if ranges[i] == 0 {
                continue;           // example 2
            }
            let st = (i as i32 - ranges[i]).max(0);
            vvi[st as usize].push(i);
        }

        // println!("{:?}", vvi);

        let mut far = 0;
        let mut nw = 0;
        let mut ans = 0;
        for i in 0..=(n as usize) {
            for j in 0..vvi[i].len() {
                far = far.max(vvi[i][j] + ranges[vvi[i][j]] as usize);
            }
            if i == nw && i != (n as usize) {
                if far == i {
                    return -1;
                }
                nw = far;
                ans += 1;
            }
        }
        return ans;
    }
}






struct Solution {}

fn main()
{
    // let n = 5;
    // // let vi = [3,4,1,1,0,0].to_vec();
    // let vi = [0,0,0,0,0,0].to_vec();

    let n = 9;
    let vi = [0,5,0,3,3,3,1,4,0,4].to_vec();

    println!("ans: {:?}", Solution::min_taps(n, vi));
}


