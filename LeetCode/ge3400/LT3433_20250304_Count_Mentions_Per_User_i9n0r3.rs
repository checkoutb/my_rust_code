






// Runtime
// 3ms
// Beats87.72%
// Memory
// 2.39MB
// Beats96.49%



// 100 event, 100 user
// ..未排序。。 先上下线，然后MSG   .. string.cmp(string) .. 字典序。
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut vtm = vec![0; number_of_users as usize];  // now >= vtm[i] => user i is online
        let mut vans = vec![0; number_of_users as usize];

        let mut events = events;
        events.sort_by(|a, b| if a[1]==b[1] {
            b[0].cmp(&a[0])
        } else {
            let aa = a[1].parse::<i32>().unwrap();
            let bb = b[1].parse::<i32>().unwrap();
            aa.cmp(&&bb) } );
        
        for i in 0..events.len() {
            let e0 = events[i][0].clone();
            let e1 = events[i][1].clone();
            let e2 = events[i][2].clone();

            let e1tm = e1.parse::<i32>().unwrap();
          
            if e0 == "MESSAGE".to_string() {
                if e2 == "ALL" {
                    for j in 0..vans.len() {
                        vans[j] += 1;
                    }
                } else if e2 == "HERE" {
                    for j in 0..vans.len() {
                        if vtm[j] <= e1tm {
                            vans[j] += 1;
                        }
                    }
                } else {
                    let s = e2.into_bytes();
                    let mut idx = -1i32;
                    for j in 0..s.len() {
                        if s[j] >= b'0' && s[j] <= b'9' {
                            if idx == -1 {
                                idx = 0;
                            }
                            idx *= 10;
                            idx += (s[j] - b'0') as i32;
                        } else {
                            if idx != -1 {
                                vans[idx as usize] += 1;
                                idx = -1;
                            }
                        }
                    }
                    vans[idx as usize] += 1;
                }
            } else {
                let e2id = e2.parse::<usize>().unwrap();
                vtm[e2id] = e1tm + 60;
            }
        }

        vans
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


