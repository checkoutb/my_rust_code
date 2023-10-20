



// Runtime35 ms
// Beats
// 100%
// Memory10.1 MB
// Beats
// 16.67%

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut vi: Vec<i32> = vec![-1; n as usize + 1];
        let mut vvi: Vec<Vec<i32>> = vec![vec![]; n as usize + 1];
        for rel in relations {
            vvi[rel[1] as usize].push(rel[0]);
        }
        let mut ans = 0;
        for i in 1..=(n as usize) {
            ans = ans.max(Self::dfs_a1(&vvi, &mut vi, &time, i));
        }
        ans
    }

    fn dfs_a1(vvi: &Vec<Vec<i32>>, vi: &mut Vec<i32>, tm: &Vec<i32>, node: usize) -> i32 {
        if vi[node] != -1 {
            return vi[node];
        }
        let mut t2 = 0i32;
        for nxt in vvi[node].iter() {
            t2 = t2.max(Self::dfs_a1(vvi, vi, tm, *nxt as usize));
        }
        vi[node] = t2 + tm[node - 1];
        vi[node]
    }
}





struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


