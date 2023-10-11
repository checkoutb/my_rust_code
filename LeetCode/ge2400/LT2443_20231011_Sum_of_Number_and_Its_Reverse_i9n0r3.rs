


// hint: The constraints are small enough that we can check every number.
// .........

// g



// abcdefg
// gfedcba
// 
// 443  073  370
// 34433    11722
// 43433    16272
// 3(13)3(13)3
//
// 12   T
// 20   F
impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        if num % 2 == 0 {
            return true;
        }
        let mut vi = Vec::new();
        let mut num = num;
        while num > 0 {
            vi.push(num % 10);
            num /= 10;
        }
        vi.reverse();
        let mut st = 0;
        let mut en = vi.len() - 1;
        while st < en {
            if vi[st] == vi[en] {
                st += 1;
                en -= 1;
                continue;
            }
            if vi[st] == vi[en] + 1 {
                vi[st] -= 1;
                vi[st + 1] += 10;
            } else if vi[st] == vi[en] + 10 {
                vi[en - 1] -= 1;
                vi[en] += 10;
            } else {
                return false;
            }
            st += 1;
            en -= 1;
        }
        if vi.len() % 2 == 1 && vi[vi.len() / 2] % 2 == 1 {
            return false;
        }
        true
    }
}




struct Solution {}

fn main()
{

    let a = 443;

    println!("ans: {:?}", Solution::sum_of_number_and_reverse(a));
}


