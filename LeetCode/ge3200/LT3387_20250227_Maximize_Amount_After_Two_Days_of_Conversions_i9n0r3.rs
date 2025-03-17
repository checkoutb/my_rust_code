







// Runtime
// 6ms
// Beats12.50%
// Memory
// 2.25MB
// Beats81.25%



use std::collections::HashMap;
impl Solution {


    pub fn max_amount(
        initial_currency: String, 
        pairs1: Vec<Vec<String>>, rates1: Vec<f64>, 
        pairs2: Vec<Vec<String>>, rates2: Vec<f64>) -> f64 {
        
        let mut ans = 1.0f64;

        let map2 = Self::cal(&pairs1, &rates1, initial_currency.clone(), 1.0);

        for (k, v) in map2 {
            let map3 = Self::cal(&pairs2, &rates2, k, v);

            ans = ans.max(*map3.get(&initial_currency).unwrap_or(&0.0));
        }

        ans
    }

    fn cal(pairs: &Vec<Vec<String>>, rates: &Vec<f64>, cur: String, num: f64) -> HashMap<String, f64> {

        let mut map2: HashMap<String, f64> = HashMap::new();
        map2.insert(cur.clone(), num);

        let mut vs = vec![];
        vs.push(cur);

        while !vs.is_empty() {
            // println!("{}", vs.len());
            let mut vs2 = vec![];
            for i in 0..vs.len() {
                // println!(" {} ", vs[i]);
                for j in 0..pairs.len() {
                    if vs[i] == pairs[j][0] {
                        let t2 = map2.get(&vs[i]).unwrap() * rates[j];
                        if !map2.contains_key(&pairs[j][1].clone()) || *map2.get(&pairs[j][1]).unwrap() < t2 {
                            // println!("     {}", map2.get(&pairs[j][1]).unwrap_or_default());
                            map2.entry(pairs[j][1].clone()).and_modify(|v| *v = t2).or_insert(t2);
                            vs2.push(pairs[j][1].clone());
                        }
                    } else if vs[i] == pairs[j][1] {
                        let t2 = map2.get(&vs[i]).unwrap() / rates[j];
                        if !map2.contains_key(&pairs[j][0].clone()) || *map2.get(&pairs[j][0]).unwrap() < t2 {
                            // println!("     {}", map2.get(&pairs[j][1]).unwrap_or_default());
                            map2.entry(pairs[j][0].clone()).and_modify(|v| *v = t2).or_insert(t2);
                            vs2.push(pairs[j][0].clone());
                        }
                    }
                }
            }
            vs = vs2;
        }
        map2
    }
}


struct Solution {}

fn main()
{

    let s = "EUR".to_string();
    
    let p1 = [["EUR".to_string(), "USD".to_string()].to_vec(), ["USD".to_string(), "JPY".to_string()].to_vec()].to_vec();
    let r1 = [2.0, 3.0].to_vec();

    let p2 = [["JPY".to_string(),"USD".to_string()].to_vec(), ["USD".to_string(),"CHF".to_string()].to_vec(), ["CHF".to_string(),"EUR".to_string()].to_vec()].to_vec();
    let r2 = [4.0, 5.0, 6.0].to_vec();

    println!("ans: {:?}", Solution::max_amount(s, p1, r1, p2, r2));
}


