

// D D


// let mut ans = vec![ Vec::new();num_rows as usize]; 
// ans[0].push(1);
// for  i in 1..num_rows as usize {
//     ans[i].push(1);
//     for j in 1..i as usize {
//         let a = ans[i-1][j-1];
//         let b = ans[i-1][j];
//         ans[i].push(a+b);
//     }
//     ans[i].push(1);
// }
// ans




// Runtime1 ms
// Beats
// 78.79%
// Memory2.2 MB
// Beats
// 32.73%
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // let vvi = vec![];
        let mut vvi = Vec::new();
        vvi.push(Vec::new());
        // vvi.push([1]);           // Vec<{integer}>
        vvi[0].push(1);
        for i in 0..(num_rows as usize - 1) {
            let mut vi = vec![1; 1];
            for j in 0..(vvi[i].len() - 1) {
                vi.push(vvi[i][j] + vvi[i][j + 1]);
            }
            vi.push(1);
            vvi.push(vi);

            // println!("{:?}", vvi);
        }
        vvi
    }
}





struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::generate(5));
}


