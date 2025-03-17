





// Runtime
// 165ms
// Beats33.33%
// Memory
// 2.24MB
// Beats66.67%



// 初始能量0， 初始能量增长速度 1/min
// 能量归零， 能量增长速度 + k

// 所以 从最小的开始？  .. example 1 过不了。。

// [25,15,15,8] 6   -> 14
// 8  kill8   x=7
// 3  15  13
// 2  15  19   /////////////
// 2  25

// 8 8 7
// 4 25 13
// 2 15 19
// 1

// 8 8 7
// 3 15 13
// 2 25 19   /////////////////
// 1 15


// [3,6,7,18,22,50] 4   -> 12
// 3 3 5
// 2 7 9
// 1 6 13
// 2 22 17
//    // 3 50 21
// 1 18

// 那就不能贪心了啊。。。


// .... n<=8 ....  8! ~= 40000


impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let mut strength = strength;
        strength.sort();
        let ans = Self::dfsa1(&strength, k, 0, 1);
        ans
    }

    fn dfsa1(strength: &Vec<i32>, k: i32, broken: i32, fx: i32) -> i32 {
        let mut ans = 0;
        for i in 0..strength.len() {
            if broken & (1 << i) == 0 {
                let mut t2 = (strength[i] + fx - 1) / fx;
                t2 += Self::dfsa1(&strength, k, broken | (1 << i), fx + k);
                if ans == 0 {
                    ans = t2;
                } else {
                    ans = ans.min(t2);
                }
            }
        }
        return ans;
    }
}


impl Solution {
    pub fn find_minimum_time___EEEE(strength: Vec<i32>, k: i32) -> i32 {
        let mut strength = strength;
        strength.sort();

        let mut ans = 0;
        let mut fx = 1;
        // for i in 0..strength.len() {
        let mut i = 0;
        while i < strength.len() {
            if strength[i] == i32::MAX {
                i += 1;
                continue;
            }

            ans += ((strength[i] + fx - 1) / fx);

            let eng = ((strength[i] + fx - 1) / fx) * fx;

            let mut j = i;
            // while j < strength.len() && strength[j] <= eng {
            //     j += 1;
            // }
            // j -= 1;
            for k in i..strength.len() {
                if strength[k] == i32::MAX {
                    continue;
                }
                if strength[k] > eng {
                    break;
                }
                j = k;
            }

            if i != j {
                strength[j] = i32::MAX;
            } else {
                i += 1;
            }

            fx += k;
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


