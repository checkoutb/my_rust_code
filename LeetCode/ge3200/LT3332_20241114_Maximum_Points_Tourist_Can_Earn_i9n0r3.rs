

// D D

// for day in (0..k).rev() {

// dp[i][dest] = max(dp[i-1][curr] + travelScore[curr][dest], dp[i-1][dest] + stayScore[i][dest])
// for (int i = 0; i < k; ++i) {
//     vector<int> dp2 = dp;
//     for (int curr = 0; curr < n; ++curr)
//         dp2[curr] += stayScore[i][curr];
//     for (int curr = 0; curr < n; ++curr)
//         for (int dest = 0; dest < n; ++dest)
//             dp2[dest] = max(dp2[dest], dp[curr] + travelScore[curr][dest]);
//     dp = move(dp2);
// }




// 没有 usize.from(i32) 

// Runtime
// 37ms
// Beats100.00%
// Analyze Complexity
// Memory
// 2.81MB
// Beats19.23%

// stay[day][city] travel[st][en]

// [city][remain day]
// 200*200 可以硬算吧？ day 从后往前算。
// ...200*200*200 。。 1kw
impl Solution {
    pub fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
        
        let mut day = k - 1;
        let mut vi = vec![0; n as usize]; // 如果day的开始时 在 [i] city
        while day >= 0 {
            let mut vi2 = vec![0; n as usize];
            for city in 0..n as usize {
                let mut t2 = stay_score[day as usize][city] + vi[city];
                for dest in 0..n as usize {
                    t2 = t2.max(travel_score[city][dest] + vi[dest]);
                }
                vi2[city] = t2;
            }
            vi = vi2;
            day -= 1;
        }
        vi.into_iter().max().unwrap()
    }
}



struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


