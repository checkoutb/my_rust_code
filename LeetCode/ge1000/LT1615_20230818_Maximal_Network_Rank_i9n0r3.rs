


// let mut connected: HashSet<(i32, i32)> = HashSet::new();
// connected.insert((u, v));
// if connected.contains(&(i, j)) {


// Runtime8 ms
// Beats
// 100%
// Memory2.4 MB
// Beats
// 100%


// max count of road that connect with 2 city.  should distinct road.
// city X has how many road
// city A, B has how many common road.   2D array
// city count <= 100
// at most one road. bool 2D array  ,, int. rust cannot ?: ..

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        // let sz1 = roads.len();

        let mut vi = vec![0; n];
        let mut vvi = vec![vec![0; n]; n];
        
        for vi2 in roads {
            let a = vi2[0] as usize;
            let b = vi2[1] as usize;
            vi[a as usize] += 1;
            vi[b as usize] += 1;
            vvi[a as usize][b as usize] += 1;
        }

        println!("{:?}", vi);
        // println!("{:#?}", vvi);
        println!("{:?}", vvi);

        let mut ans = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                ans = ans.max(vi[i] + vi[j] - vvi[i][j] - vvi[j][i]);
            }
        }
        ans
    }
}




struct Solution {}

fn main()
{

    let vvi = [[0,1].to_vec(),[0,3].to_vec(),[1,2].to_vec(),[1,3].to_vec()].to_vec();
    let n = 4;

    println!("ans: {:?}", Solution::maximal_network_rank(n, vvi));
}


