






// D

// sort by age.



// g    ai,zhiqianbuhui,xianzaihaishibuhui.



// 年龄大，score 不能小于

// age < 1000

// dp:  vi[age]  最大age， value是最大score sum

// sort by score

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut vii = vec![(0, 0); scores.len()];
        for i in 0..scores.len() {
            vii[i].0 = scores[i];
            vii[i].1 = ages[i];
        }
        vii.sort();

        let mut vi = vec![0; 1001];  // [age]
        let mut ans = 0;

        for i in 0..vii.len() {
            let (sc, ag) = vii[i];
            if i > 0 && ag == vii[i - 1].1 {
                vi[ag as usize] += sc;
            }
            let mut sum2 = 0;
            for j in 1..ag as usize {
                sum2 += vi[j];
            }
            vi[ag as usize] += sc;
            // sum2 += vi[ag as usize];
            sum2 += sc;
            ans = ans.max(sum2);
        }
        
        // for i in 0..vii.len() {
        //     let (sc, ag) = vii[i];
        //     if i > 0 && ag == vii[i - 1].1 {
        //         vi[ag as usize] += sc;
        //         continue;
        //     }
        //     let mut sum2 = sc;
        //     for j in 1..ag as usize {
        //         sum2 += vi[j];
        //     }
        //     vi[ag as usize] = vi[ag as usize].max(sum2);
        // }

        // let mut ans = 0;
        // for i in 0..vi.len() {
        //     ans = ans.max(vi[i]);
        // }
        ans
    }
}


struct Solution {}

fn main()
{

    let vi = [4,5,6,5].to_vec();
    let v2 = [2,1,2,1].to_vec();

    println!("ans: {:?}", Solution::best_team_score(vi, v2));
}


