




// g

// bs
// 但 怎么计算 最少需要 几次？ 就是 往左消除 还是 往右消除？ 贪心？ 消除的区间重叠呢？ 碰到重叠，直接返回就可以了。
//      是 往左，然后重叠好，还是往右，不重叠好。  得dp啊。

impl Solution {
    pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut st = nums[0];
        let mut en = nums[0];
        for n in nums {
            st = st.min(n);
            en = en.max(n);
        }
        let mut ans = st;
        while st <= en {
            let md = (st + en) / 2;
            if can(&nums, k, md) {
                ans = md;
                st = md + 1;
            } else {
                en = md - 1;
            }
        }
        ans
    }

    fn can(vi: &Vec<i32>, k: i32, mxv: i32) -> bool {
        let mut vp : Vec<(i32, i32)> = Vec::new();      // <to left, to right>
        let mut lsti = -1;
        let mut lsten = -1;

    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


