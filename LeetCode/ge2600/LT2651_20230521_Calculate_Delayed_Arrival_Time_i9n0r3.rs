







impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::find_delayed_arrival_time(15, 5));
}


