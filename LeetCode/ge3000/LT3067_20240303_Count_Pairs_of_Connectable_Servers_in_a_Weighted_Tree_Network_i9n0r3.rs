





// not enough submission

// Runtime
// 80ms
// Beats100.00%of users with Rust
// Memory
// 2.52MB
// Beats100.00%of users with Rust

// 想不出。。 hint1。。  each node as root, dfs

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let sz1 = edges.len();
        let mut vvi = vec![Vec::new(); sz1 + 1];
        for vi in edges {
            vvi[vi[0] as usize].push((vi[1], vi[2]));
            vvi[vi[1] as usize].push((vi[0], vi[2]));
        }
        let mut ans = vec![0; sz1 + 1];
        for i in 0..=sz1 {
            ans[i] = Self::calc(i, &vvi, signal_speed);
        }
        ans
    }

    fn calc(node: usize, vvi: &Vec<Vec<(i32, i32)>>, divd: i32) -> i32 {
        // let mut vi = vec![];
        let mut cnt = 0;
        let mut ans = 0;
        for i in 0..vvi[node].len() {
            let mut vi = vec![];
            Self::dfsa1(vvi[node][i].0 as usize, node, vvi, &mut vi, vvi[node][i].1);
            let mut t2 = 0;
            // println!("{:?}", &vi);
            for dis in vi {
                if dis % divd == 0 {
                    t2 += 1;
                }
            }
            // println!("{}, {}, {}, {}", node, t2, cnt, ans);
            ans += t2 * cnt;
            cnt += t2;
        }
        ans
    }

    fn dfsa1(node: usize, parent: usize, vvi: &Vec<Vec<(i32, i32)>>, vi: &mut Vec<i32>, dis: i32) {
        vi.push(dis);
        for i in 0..vvi[node].len() {
            if vvi[node][i].0 as usize == parent {
                continue;
            }
            Self::dfsa1(vvi[node][i].0 as usize, node, vvi, vi, dis + vvi[node][i].1);
        }
    }
}



struct Solution {}

fn main()
{


    // println!("ans: {:?}", Solution::());
}


