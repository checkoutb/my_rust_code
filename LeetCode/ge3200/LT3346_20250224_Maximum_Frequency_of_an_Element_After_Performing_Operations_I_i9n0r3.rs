




// hint: Sort the array and try each value in range as a candidate...
// 但愿下次能记得。

// g




// 选出 numOp 个元素，这些元素 -k 到 k。 计算最大的 frequency

// sort, 区间？   arr[i] + 2*k.  2 pointer.
// ==2k <2k ? ==2k    1+2*1 = 3,   1->2,  3->2
// min(range.len, numOp)

// ... 问题出在 min。 并不是 所有range中元素 都需要修改的。。我的发。
// 还得追踪 range中 原始的 最大的 frequency。 挺麻烦的。 似乎只能map硬算啊。
// .....还是不对。 不能用 原始的最大的frequency。。 是要使用 arr[i] + k。 但是 Example 2 也没有办法。

// 所以 应该 以[i]为中心，向两侧 扩展 k。
// ....[53,88], 27, 2  ..又要回去了。。 我又有了一个idea。。。


// range query max ..

// [1,90], 76, 1  .... 我。。


impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort();
        let mut vii = vec![];

        let mut cnt = 1;
        for i in 1..nums.len() {
            if (nums[i] != nums[i - 1]) {
                vii.push((nums[i - 1], cnt));
                cnt = 1;
            } else {
                cnt += 1;
            }
        }
        vii.push((nums[nums.len() - 1], cnt));

        let mut idx = 0;
        for i in 0..nums.len() {
            
        }
        ans as i32
    }

    fn bsaaa_2(vii: &Vec<(i32, i32)>, val: i32) -> usize {
        let mut st = 0;
        let mut en = vii.len() - 1;
        while st < en {
            let md = (st + en + 1) / 2;
            // println!("{} {} {}", st, en, md);
            if vii[md].0 > val {
                en = md - 1;
            } else {
                st = md;
            }
        }
        st
    }
}


impl Solution {
    pub fn max_frequency__EER(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::collections::HashMap;
        let mut map2: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        let mut nums = nums;
        nums.sort();

        for i in &nums {
            map2.entry(*i).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut vii = vec![(0,0); map2.len()];
        let mut idx = 0;
        for (k, v) in map2.iter() {
            vii[idx].0 = *k;
            vii[idx].1 = *v;
            idx += 1;
        }

        vii.sort();

        idx = 0;
        for i in 0..nums.len() {
            while idx < nums.len() && nums[idx] <= nums[i] + 2 * k {
                idx += 1;
            }

            let mut mx = nums[i] + k;
            let mut mn = nums[idx - 1] - k;
            mx = mx.min(nums[idx - 1]);
            mn = mn.max(nums[i]);

            let st = Self::bsaaa(&vii, mn);
            let en = Self::bsaaa(&vii, mx);

            let mut cnt = 0;
            for j in st..=en {
                cnt = cnt.max(vii[j].1);
            }

            println!("{} {} {}", idx, i, cnt);
            if idx - i >= cnt as usize {
                ans = ans.max(cnt as usize + (idx - i - cnt as usize).min(num_operations as usize));
            }
        }

        ans as i32
    }

    fn bsaaa(vii: &Vec<(i32, i32)>, val: i32) -> usize {
        let mut st = 0;
        let mut en = vii.len() - 1;
        while st < en {
            let md = (st + en + 1) / 2;
            // println!("{} {} {}", st, en, md);
            if vii[md].0 > val {
                en = md - 1;
            } else {
                st = md;
            }
        }
        st
    }
}



impl Solution {
    pub fn max_frequency__EE(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut pfxi = 0;
        let mut sfxi = 0;
        let mut t2 = 0;
        let mut cnt = 0;
        // for i in 0..nums.len() {
        let mut i = 0;
        let mut ans = 0;
        while i < nums.len() {
            cnt = 1;
            while i + 1 < nums.len() && nums[i] == nums[i + 1] {
                i += 1;
                cnt += 1;
            }
            t2 = nums[i] - k;
            while nums[pfxi] < t2 {
                pfxi += 1;
            }
            t2 = nums[i] + k;
            while sfxi < nums.len() && nums[sfxi] <= t2 {
                sfxi += 1;
            }

            // [pfxi, sfxi)

            ans = ans.max(cnt + (sfxi - pfxi - cnt).min(num_operations as usize));

            i += 1;
        }
        ans as i32
    }
}



impl Solution {
    pub fn max_frequency____EEEE(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {

        use std::collections::HashMap;

        let mut nums = nums;
        nums.sort();
        let mut idx = 0usize;
        let numOp2 = 2 * num_operations;
        let mut ans = 0;
        let numOpU = num_operations as usize;
        let mut not_change = 0;
        let mut map2: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let ori_idx = idx;
            if i > 0 {
                map2.entry(nums[i - 1]).and_modify(|v| *v -= 1);
                if *map2.get(&nums[i - 1]).unwrap() == 0 {
                    map2.remove(&nums[i - 1]);
                }
            }
            
            while idx < nums.len() && nums[idx] <= nums[i] + numOp2 {
                map2.entry(nums[idx]).and_modify(|v| *v += 1).or_insert(1);
                idx += 1;
            }
            // idx is first ele that not in range
            if ori_idx == idx {
                continue;
            }

            not_change = 0;
            for &cnt in map2.values() {
                not_change = not_change.max(cnt);
            }

            ans = ans.max((idx - i).min(numOpU + not_change as usize));
        }
        ans as i32
    }
}



struct Solution {}

fn main()
{

    // let vi = [5,11,22,22].to_vec();
    // let k = 5;
    // let cnt = 1;

    let vi = [1,90].to_vec();
    let k = 76;
    let cnt = 1;

    println!("ans: {:?}", Solution::max_frequency(vi, k, cnt));
}


