





// D D

// 0-20亿， 最多60次


// Runtime
// 85ms
// Beats56.76%
// Memory
// 7.38MB
// Beats21.62%


// g.   error  .. 抄了最多60次，导致 ans 没有赋值。。 所以增加了 ans=md

// tle... 抄了 Solution的 最多60次。

// ? gap 怎么处理

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut vi = vec![];

        let mut squares = squares;
        // squares.sort_by(|a, b| if a[1] + a[2] == b[1] + b[2] {
        //     b[1].cmp(&a[1])
        // } else {
        //     let t2 = b[1] + b[2];
        //     (a[1] + a[2]).cmp(&t2)
        // });

        squares.sort_by_key(|a| a[1]);

        let mut st = squares[0][1];
        let mut en = squares[0][1] + squares[0][2];

        let mut varea = vec![0f64; squares.len()];
        for i in 0..squares.len() {
            if squares[i][1] > en {
                vi.push((st, en));
                st = squares[i][1];
                en = squares[i][1] + squares[i][2];
            } else {
                en = en.max(squares[i][1] + squares[i][2]);
            }
            
            varea[i] = squares[i][2] as f64 * squares[i][2] as f64;
        }

        vi.push((st, en));

        println!("{:?}", varea);
        println!(" --  {:?}", vi);

        let mut st2 = vi[0].0 as f64;
        let mut en2 = vi[vi.len() - 1].1 as f64;
        let mut ans = 0f64;
        let mut cnt = 0;
        while st2 <= en2 {
            cnt += 1;
            if cnt > 60 {
                break;
            }
            let md = (st2 + en2) / 2 as f64;
            let mut uparea = 0f64;
            let mut downarea = 0f64;
            for i in 0..squares.len() {
                if md > (squares[i][1] + squares[i][2]) as f64 {
                    downarea += varea[i];
                } else if md < (squares[i][1] as f64) {
                    uparea += varea[i];
                } else {
                    let t2 = varea[i] * (md - squares[i][1] as f64) / (squares[i][2] as f64);
                    downarea += t2;
                    uparea += varea[i] - t2;
                }

            }
            println!("----{} {} {}", md, downarea, uparea);
            println!("{} {} {}", st2, en2, ans);
            println!("{:?}", varea);
            println!(" --  {:?}", vi);
            println!("{:?}", squares);

            
            if (downarea - uparea).abs() < 0.000001 {
                ans = md;
                st2 = en2 + 1f64;
            } else if downarea > uparea {
                en2 = md;
            } else {
                st2 = md;
                ans = md;   // ......
            }
        }

        for i in 1..vi.len() {
            if vi[i].0 as f64 > ans {

                if ans > vi[i - 1].1 as f64 {
                    ans = vi[i - 1].1 as f64
                }

                break;
            }
        }
        ans
    }
}


struct Solution {}

fn main()
{

    // let vvi = [[0,0,2].to_vec(), [1,1,1].to_vec()].to_vec();

    // let vvi = [[26,28,2].to_vec(), [16,23,2].to_vec()].to_vec();
    // let vvi = [[15,24,3].to_vec(),[5,20,3].to_vec()].to_vec();

    // let vvi = [[12,18,8].to_vec(),[23,24,2].to_vec()].to_vec();

    let vvi = [[12,13,15].to_vec(),[19,23,1].to_vec()].to_vec();

    println!("ans: {:?}", Solution::separate_squares(vvi));
}


