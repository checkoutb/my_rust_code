





// 吐了。。

// Runtime1 ms
// Beats
// 82.14%
// Memory2.1 MB
// Beats
// 35.71%

pub fn lta(n: i32) -> Vec<Vec<i32>> {
    
    let mut t2 = n * n;
    let mut mni: i32 = 0;
    let mut mxi = n - 1;
    let mut mnj = 0;
    let mut mxj = n - 1;

    let n: usize = n as usize;
    let mut ans = vec![vec![0; n]; n];
    // ans[1][1] = 1;
    // println!("{}", ans[1][1]);

    let mut x : i32 = 0;
    let mut y : i32 = 0;

    let arr: Vec<i32> = [0,1,0,-1,0].to_vec();
    let mut dir = 0;
    let mut num = 1;
    while t2 > 0 {
        t2 -= 1;
        ans[x as usize][y as usize] = num;
        num += 1;

        let x2 = x + arr[dir];
        let y2 = y + arr[dir + 1];
        
        if x2 < mni || x2 > mxi || y2 < mnj || y2 > mxj {
            if dir == 0 {
                mni += 1;
            } else if dir == 1 {
                mxj -= 1;
            } else if dir == 2 {
                mxi -= 1;
            } else {
                mnj += 1;
            }
            dir = (dir + 1) % 4;
        }
        x = x + arr[dir];
        y = y + arr[dir + 1];
    }

    ans
}


fn main()
{
    // println!("ok");
    lta(4);
}


