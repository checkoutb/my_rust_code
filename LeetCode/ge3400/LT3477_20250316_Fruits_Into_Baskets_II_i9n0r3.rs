








// Runtime
// 0ms
// Beats100.00%
// Memory
// 2.22MB
// Beats86.27%




// <<<<<<
// len < 101

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let sz1 = fruits.len();
        let mut vb = vec![false; sz1];

        let mut ans = sz1 as i32;
        for i in 0..sz1 {
            let t2 = fruits[i];
            for j in 0..sz1 {
                if baskets[j] >= t2 && !vb[j] {
                    ans -= 1;
                    vb[j] = true;
                    break;
                }
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


