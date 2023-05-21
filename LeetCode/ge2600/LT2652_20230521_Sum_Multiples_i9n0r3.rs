


// D D
// (1..=n).filter(|it| it % 3 == 0 || it % 5 == 0 || it % 7 == 0).sum::<i32>()



impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut ans: i32 = 0;
        for i in 1..=n {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                ans += i;
            }
        }
        ans
    }
}




struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::sum_of_multiples(7));
}


