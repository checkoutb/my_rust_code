
// use std::fmt::Display;

// impl Display for Vec<(i32, i32)> {

// }





// D D

/*

unreachable!()

let mut queue = std::collections::VecDeque::new();

queue.push_back((i, j, 0 as i32));

let (x, y, level) = queue.pop_front().unwrap();

while !queue.is_empty() {

let (x, y) = (x as usize, y as usize);
if grid[x][y] != 1 {
    return;
}

'outer: for i in 0..grid.len() {
    for j in 0..grid[0].len() {
        if grid[i][j] == 1 {
            Self::mark(&mut grid, i as i32, j as i32);
            break 'outer;
        }
    }
}



let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();

while q.len() > 0 {
    if let Some((row, col, flips)) = q.pop_front() {
        let directions:Vec<(i32, i32)> = vec![(0,1),(1,0),(-1,0),(0,-1)];

        for dir in directions.iter(){
            let (nr, nc) = (row + dir.0 as usize, col + dir.1 as usize);



let mut land = VecDeque::new();
let (m, n) = (grid.len()-1, grid[0].len()-1);
'outer: for x in 0..=m {
    for y in 0..=n {

*/








// Runtime17 ms
// Beats
// 10%
// Memory2.9 MB
// Beats
// 20%

// ??? grid 是不可变的吧。不是。 .. 不可变的。



impl Solution {

pub fn lta(grid: Vec<Vec<i32>>) -> i32 {
    let sz1 = grid.len() as i32;
    let mut vst = vec![vec![false; sz1 as usize]; sz1 as usize];

    let mut vv: Vec<(i32, i32)> = vec![];
    'AAA:
    for i in 0..sz1 {
        for j in 0..sz1 {
            if grid[i as usize][j as usize] == 1 && vst[i as usize][j as usize] == false {
                vv = Self::dfsa1(&grid, i, j, sz1, &mut vst).unwrap();
                break 'AAA;
            }
        }
    }
    
    // println!("{:?}", &grid);
    // println!("{:?}", &vst);

    let mut vv2: Vec<(i32, i32)> = vec![];
    for i in 0..sz1 {
        for j in 0..sz1 {
            if grid[i as usize][j as usize] == 1 && !vst[i as usize][j as usize] {
                vv2 = Self::dfsa1(&grid, i, j, sz1, &mut vst).unwrap();
                break;
            }
        }
    }
    let mut ans : i32 = 123123123;

    // println!("vv : {:?}", vv.len());
    // println!("vv2 : {}", vv2.len());

    // println!("{}", vv);
    // println!("{}", vv2);

    for t2 in vv {
        for t3 in &vv2 {
            if ans > (t2.0 - t3.0).abs() + (t2.1 - t3.1).abs() {
                ans = (t2.0 - t3.0).abs() + (t2.1 - t3.1).abs()
            }
        }
    }

    return ans - 1;
}



fn dfsa1(vvi: &Vec<Vec<i32>>, i: i32, j: i32, sz1: i32, vst: &mut Vec<Vec<bool>>) -> Option<Vec<(i32, i32)>> {
    if i < 0 || j < 0 || i == sz1 || j == sz1 || vst[i as usize][j as usize] || vvi[i as usize][j as usize] == 0 {
        return None;
    }
    
    // println!(" {} .. {} ", i, j);

    vst[i as usize][j as usize] = true;
    let mut ans : Vec<(i32, i32)> = vec![];

    let arr = [1,0,-1,0,1].to_vec();
    let mut need = false;
    let mut idx: usize = 1;
    while idx < 5 {
        if i + arr[idx] >= 0 && i + arr[idx] < sz1 && j + arr[idx - 1] >= 0 && j + arr[idx - 1] < sz1 
        && vvi[(i + arr[idx]) as usize][(j + arr[idx - 1]) as usize] == 0 {
            need = true;
        }
        
        if let Some(mut ans2) = Self::dfsa1(vvi, i + arr[idx], j + arr[idx - 1], sz1, vst) {
            ans.append(&mut ans2);
        }

        // let mut ans2 = dfsa1(vvi, i + arr[idx], j + arr[idx - 1], sz1);
        // match ans2 {
        //     Some(ans3) => ans.append(&mut ans2),
        //     None => ;,
        // }
        idx += 1;
    }
    if need {
        ans.push((i, j));
    }

    return Some(ans);
}

}


fn main()
{

    //let mut arr = {{1,0},{0,1}};
    let mut arr = vec![vec![0; 2]; 2];
    arr[0][0] = 1;
    arr[1][1] = 1;

    // println!("ans: {}", lta(arr));

    //let mut t2 : Solution;

    println!("ans: {}", Solution::lta(arr));

}

struct Solution {}

