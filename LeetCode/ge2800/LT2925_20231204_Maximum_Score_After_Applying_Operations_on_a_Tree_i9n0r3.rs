





// Runtime47 ms
// Beats
// 25%
// Memory6.5 MB
// Beats
// 64.29%

impl Solution {
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let sz1 = values.len();
        let mut vvi = vec![Vec::new(); sz1];

        for vi in &edges {
            vvi[vi[0] as usize].push(vi[1]);
            vvi[vi[1] as usize].push(vi[0]);
        }

        let mn_remain = Self::dfsa1(&vvi, &values, 0, -1);
        let sum2 = Self::sum_of_node(&vvi, &values, 0, -1);

        sum2 - mn_remain
    }

    // min remain in tree == max score
    fn dfsa1(vvi: &Vec<Vec<i32>>, vi: &Vec<i32>, node: i32, parent: i32) -> i64 {
        let node = node as usize;
        let mut ans = 0i64;

        for &nxt in &vvi[node] {
            if nxt == parent { continue; }
            ans += Self::dfsa1(vvi, vi, nxt, node as i32);
        }

        if ans == 0 {
            ans = vi[node] as i64;     // leaf
        } else {
            ans = ans.min(vi[node] as i64);
        }

        ans
    }

    fn sum_of_node(vvi: &Vec<Vec<i32>>, vi: &Vec<i32>, node: i32, parent: i32) -> i64 {
        let node = node as usize;
        let mut ans = vi[node] as i64;
        
        for &nxt in &vvi[node] {
            if nxt == parent { continue; }
            ans += Self::sum_of_node(vvi, vi, nxt, node as i32);
        }
        return ans;
    }
}



struct Solution {}

fn main()
{

    let vvi = [[0,1].to_vec(),[0,2].to_vec(),[0,3].to_vec(),[2,4].to_vec(),[4,5].to_vec()].to_vec();
    let vi = [5,2,5,2,1,1].to_vec();


    println!("ans: {:?}", Solution::maximum_score_after_operations(vvi, vi));
}


