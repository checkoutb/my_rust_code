










// Runtime
// 59ms
// Beats12.50%
// Memory
// 4.66MB
// Beats89.29%



// 1 6 7 8, 9 9 

impl Solution {
    pub fn max_weight(pizzas: Vec<i32>) -> i64 {
        let mut pizzas = pizzas;
        pizzas.sort();
        pizzas.reverse();
        let mut cnt = (pizzas.len() / 4) as i32;
        let mut cnt2 = cnt / 2 + if cnt % 2 == 1 {1} else {0};

        let mut idx = 0;
        let mut ans = 0i64;
        while cnt2 > 0 {
            cnt2 -= 1;
            ans += pizzas[idx] as i64;
            idx += 1;
        }

        cnt2 = cnt / 2;
        idx += 1;
        while cnt2 > 0 {
            cnt2 -= 1;
            ans += pizzas[idx] as i64;
            idx += 2;
        }
        ans
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


