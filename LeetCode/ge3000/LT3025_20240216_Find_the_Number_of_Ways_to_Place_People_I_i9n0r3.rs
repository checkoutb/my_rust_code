

// D

// ......
// x升序，y降序。 遍历每个点，对于每个点，向后搜索，如果 y 大于之前的 mxy 且 y小于等于点的y。 更新mxy




// g


// [[1,5],[2,0],[5,5]]

// [[1,5],[2,0],[5,5]]
// [[1,4],[1,5],[0,3]]
// [[0,1],[1,3],[6,1]]      2

// ... sort by x, then sweep line ?  2 pointer,, vvi is better, 2 pointer is harder.. still need 2 pointer... but with vvi, 2 pointer easier
// error.

impl Solution {


    // II n < 1000
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort();
        let mut vvi = Vec::new();
        let mut lstx = -1;
        for vi in points {
            if vi[0] != lstx {
                vvi.push(Vec::new());
                lstx = vi[0];
            }
            let t2 = vvi.len() - 1;
            vvi[t2].push(vi[1]);
        }
        for i in 0..vvi.len() {
            vvi[i].reverse();
        }
        vvi.reverse();
        let mut ans = vvi[0].len() as i32 - 1;
        let mut vi = vvi[0].clone();
        let mut idx;
        for i in 1..vvi.len() {
            idx = 0usize;
            ans += vvi[i].len() as i32 - 1;
            for j in 0..vvi[i].len() {
                while idx < vi.len() && vi[idx] > vvi[i][j] {
                    idx += 1;
                }
                
                if idx != vi.len() {
                    ans += if j + 1 == vvi[i].len() || vi[idx] > vvi[i][j + 1] { 1 } else { 0 };
                    vi.insert(idx, vvi[i][j]);
                    idx += 1;
                } else {
                    vi.push(vvi[i][j]);
                }
            }
        }
        ans
    }


    pub fn _error_3_number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        // points.sort_by_key(|vi| vi[0] * 100 + (vi[1]));      // not suitable for II.. sort() is enough..
        points.sort();

        println!("{:?}", points);

        let mut vvi = Vec::new();
        let mut lstx = -1;
        for vi in points {
            if vi[0] != lstx {
                vvi.push(Vec::new());
                lstx = vi[0];
            }
            let t2 = vvi.len() - 1;
            vvi[t2].push(vi[1]);
        }
        // for mut vi in &vvi {
        //     vi.reverse();
        // }
        for i in 0..vvi.len() {
            vvi[i].reverse();
        }
        let mut ans = vvi[0].len() as i32 - 1;      // line
        // let mut idx1;
        let mut idx2;


        println!("{:?}", vvi);

        for i in 1..vvi.len() {
            ans += vvi[i].len() as i32 - 1;
            // idx1 = 0usize;      // [i - 1][idx1]        // alice
            idx2 = 0usize;      // [i][idx2]        // bob

            for j in 0..vvi[i - 1].len() {     // [i-1][j] alice
                while idx2 < vvi[i].len() && vvi[i][idx2] > vvi[i - 1][j] {
                    idx2 += 1;
                }
                if idx2 == vvi[i].len() {
                    // ans += if j == vvi[i - 1].len() - 1 { 0 } else { 1 };
                    ans += 0;
                } else {
                    ans += if j + 1 == vvi[i - 1].len() || vvi[i][idx2] <= vvi[i - 1][j + 1] { 1 } else { 0 };
                }
            }
        }

        ans
        
        // let mut idx = 0;        // last x
        // let mut ans = 0;
        // while idx + 1 < 
    }



    pub fn _error_2_number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|vi| vi[1] * 100 + (80 - vi[0]));        // y up, then x down
        let mut stk : Vec<i32> = Vec::new();
        let mut ans = 0;
        for vi in points {
            let (x, _y) = (vi[0], vi[1]);
            while !stk.is_empty() && stk[stk.len() - 1] >= x {
                ans += 1;
                stk.pop();
            }
            stk.push(x);
        }
        ans
    }

    pub fn _error_number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|vi| vi[1] * 100 + (80-vi[0]));
        // println!("{:?}", points);

        let mut mxx = -1;
        let mut ans = 0;
        let mut b2 = false;
        for vi in points {
            let (x, _) = (vi[0], vi[1]);
            if mxx >= x {
                ans += 1;
                if b2 {
                    ans += 1;
                    b2 = false;
                }
            } else {
                if mxx != -1 {
                    b2 = true;
                }
                mxx = x;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    // let vvi = [[6,2].to_vec(), [4,4].to_vec(), [2,6].to_vec()].to_vec();
    let vvi = [[0,1].to_vec(),[1,3].to_vec(),[6,1].to_vec()].to_vec();

    println!("ans: {:?}", Solution::number_of_pairs(vvi));
}


