










// Runtime
// 6ms
// Beats90.70%
// Memory
// 4.27MB
// Beats81.40%




impl Solution {
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {

        let mut mxgp = 0;
        for i in 0..groups.len() {
            mxgp = mxgp.max(groups[i]);
        }

        let mut vi = vec![-1i32; mxgp as usize + 1];

        for i in 0..elements.len() {
            if elements[i] > mxgp || vi[elements[i] as usize] != -1 {
                continue;
            }
            let mut t2 = elements[i] as usize;
            while t2 <= mxgp as usize {
                if vi[t2] == -1 {
                    vi[t2] = i as i32;
                }
                t2 += elements[i] as usize;
            }
        }

        let mut vans = vec![-1; groups.len()];
        for i in 0..groups.len() {
            vans[i] = vi[groups[i] as usize];
        }
        vans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


