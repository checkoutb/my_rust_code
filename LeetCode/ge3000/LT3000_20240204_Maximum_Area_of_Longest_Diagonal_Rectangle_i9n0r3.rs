



// dimensions.into_iter().for_each(|d| {
//     let diagonal = d[0] * d[0] + d[1] * d[1];
//     let area = d[0] * d[1];
//     match diagonal.cmp(&max_diagonal) {
//         Greater => (max_diagonal, max_area) = (diagonal, area),
//         Equal => max_area = max_area.max(area),
//         _ => (),
//     }
// });



// AC


impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut mxdiag = 0;
        for vi in dimensions.into_iter() {
            let t2 = vi[0] * vi[0] + vi[1] * vi[1];
            if t2 > mxdiag {
                mxdiag = t2;
                ans = vi[0] * vi[1];
            } else if t2 == mxdiag {
                ans = ans.max(vi[0] * vi[1]);
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{
    // let vvi = [[9,3].to_vec(), [8,6].to_vec()].to_vec();

    println!("ans: {:?}", Solution::area_of_max_diagonal(vvi));
}


