











// Runtime
// 3ms
// Beats83.33%
// Memory
// 5.13MB
// Beats81.25%



impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let sz1 = start_time.len();
        let mut vi = vec![0; sz1];  // max gap after idx

        let mut lst = event_time;
        for i in (0..sz1).rev() {
            vi[i] = lst - end_time[i];
            if i + 1 < sz1 && vi[i] < vi[i + 1] {
                vi[i] = vi[i + 1];
            }
            lst = start_time[i];
        }
        let mut mxgap = 0; // max gap before before idx
        let mut ans = 0;

        // println!("{:?}", vi);
        lst = 0;
        for i in 0..sz1 {
            let t2 = end_time[i] - start_time[i];
            if t2 <= mxgap || (i + 1 < sz1 && t2 <= vi[i + 1]) {
                // if i + 1 == sz1
                ans = ans.max((if i + 1 == sz1 { event_time } else { start_time[i + 1] }) - lst);
            } else {
                ans = ans.max((if i + 1 == sz1 { event_time } else { start_time[i + 1] }) - lst - t2);
            }

            mxgap = mxgap.max(start_time[i] - lst);
            lst = end_time[i];

            // println!("{} {} {}", i, mxgap, ans);
        }
        ans
    }
}


struct Solution {}

fn main()
{

    let en = 5;
    let vst = [1,3].to_vec();
    let ven = [2,5].to_vec();


    println!("ans: {:?}", Solution::max_free_time(en, vst, ven));
}


