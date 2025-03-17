









// Runtime
// 3ms
// Beats25.00%
// Memory
// 4.66MB
// Beats60.71%




// max sum of k elements

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut vi = vec![0; start_time.len() + 1];
        let mut lst = 0;
        for i in 0..start_time.len() {
            let st = start_time[i];
            let en = end_time[i];
            vi[i] = st - lst;
            lst = en;
        }
        vi[start_time.len()] = event_time - end_time[end_time.len() - 1];

        let mut ans = 0;
        let mut st = 0;
        let mut sum2 = 0;
        for i in 0..vi.len() {
            sum2 += vi[i];
            if i - st > k as usize {
                sum2 -= vi[st];
                st += 1;
            }
            ans = ans.max(sum2);
        }
        

        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


