


// D D

// map<vector<int>,int> r;

// let mut map: HashMap<Vec<i32>, i32> = HashMap::new();


// int k = 0;
// while (k < n && g[i][k] == g[k][j])
//     ++k;
// res += k == n;




// impl Solution {
//     pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        
//         let mut count = 0;
//         let mut row = vec![];
//         let mut col = vec![];
//         for i in 0..grid.len() {
//             row.push(grid[i].to_vec());
//             col.push(vec![]);
//             for j in 0..grid[i].len() {
//                 col[i].push(grid[j][i]);
//             }
//         }
//         for i in 0..grid.len() {
//             for j in 0..grid[i].len() {
//                 if row[i] == col[j] {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }











// Runtime31 ms
// Beats
// 26.32%
// Memory2.5 MB
// Beats
// 100%
impl Solution {

    // n <= 200
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;
        let sz1: usize = grid.len();
        for i in 0..sz1 {
            'AAA:
            for j in 0..sz1 {
                for k in 0..sz1 {
                    if grid[i][k] != grid[k][j] {
                        continue 'AAA;
                    }
                }
                ans += 1;
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


