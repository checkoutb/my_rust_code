

// D D
// 2 pri_que
// 1 pri_que

// 思路差不多，都是 需要时，选择 end最远的 query。
// 实现差太多了。。


// Runtime
// 83ms
// Beats7.14%
// Memory
// 11.21MB
// Beats21.43%






// dec [] most 1

// all 0

// del most query/op, array is still all 0

// range max query ? 不行，更新的时候是 区域的。
// 想的是： 先 应用query 到 nums，区间全部减去1.
// 按 query的 end 升序， query的 区间内 是否都 是 负数，都是负数 那么这个区间可以移除。
// 但是会tle。 应用query 可以差分， 但是 移除query 没办法差分啊。
//
// 2次差分？ 应用query， 删除query 就是 区间全部+1
// 按 start升序， end 用 pri_q保存。
//
// 1次差分？ start升序，end max_heap，  如果 当前值 大于0，说明需要一个 query，那么选择 end 最大的 query
impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut mxhp = BinaryHeap::new();
        let mut mnhp = BinaryHeap::new();
        let mut mnhp2: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        let mut queries = queries;
        queries.sort_by(|a, b| {
            if a[0] == b[0] {
                (b[1]).cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        // println!("{:?}", queries);


        
        let mut del = 0;
        let mut canDel = 0;
        let mut qidx = 0usize;
        let mut totalUsed = 0;
        for i in 0..nums.len() {
            while qidx < queries.len() && queries[qidx][0] <= i as i32 {
                // del += 1;
                mxhp.push(queries[qidx][1]);
                mnhp.push(Reverse(queries[qidx][1]));
                canDel += 1;
                qidx += 1;
            }

            while mnhp.len() > 0 && mnhp.peek().unwrap().0 < i as i32 {
                if mnhp2.len() > 0 
                    && (mnhp.peek().unwrap().0 == mnhp2.peek().unwrap().0) {
                    
                    mnhp2.pop();
                    mnhp.pop();
                    del -= 1;  /// ---  mnhp2 是使用的query，即 del的范围，所以这里 出del的范围就 把del--
                    canDel -= 1;
                } else {
                    mnhp.pop();
                    canDel -= 1;   // canDel == mnhp.len()
                    // del -= 1;
                }
            }

            // if canDel < del {
            //     del = canDel;
            // }

            let t2 = nums[i];

            if t2 <= del {
                
            } else {
                if t2 > canDel {
                    // println!("{} {} {}", i, t2, canDel);
                    return -1;    ////////////////
                } else {
                    while t2 > del {

                        // println!("--del end: {}", mxhp.peek().unwrap());

                        mnhp2.push(Reverse(mxhp.pop().unwrap()));
                        // mxhp.pop();
                        del += 1;
                        totalUsed += 1;
                        // canDel -= 1;  // ... need delete mnhp's max ele..
                        // println!("--- {}, {}", i, totalUsed);
                    }
                }
            }
        }

        queries.len() as i32 - totalUsed
    }
}

struct Solution {}

fn main()
{

    // let vi = [1,1,1,1].to_vec();
    // let vvi = [[1,3].to_vec(), [0,2].to_vec(), [1,3].to_vec(), [1,2].to_vec()].to_vec();

    let vi = [2,0,2].to_vec();
    let vvi = [[0,2].to_vec(), [0,2].to_vec(), [1,1].to_vec()].to_vec();

    println!("ans: {:?}", Solution::max_removal(vi, vvi));
}


