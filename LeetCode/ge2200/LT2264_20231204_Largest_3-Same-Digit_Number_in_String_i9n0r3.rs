


// D D

// let good_integer = num_chars[i].to_string().repeat(3);
// if good_integer > result {



// Runtime0 ms
// Beats
// 100%
// Memory2 MB
// Beats
// 78.95%
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.as_bytes();
        let mut au8 = b'!';

        for i in 2..num.len() {
            if num[i] == num[i - 1] && num[i - 1] == num[i - 2] && num[i] > au8 {
                au8 = num[i];
            }
        }
        let mut ans : String = String::new();
        if au8 != b'!' {
            for _i in 0..3 {
                ans.push(char::from_u32((au8 as usize) as u32).unwrap());
            }
        }
        return ans;
    }
}



struct Solution {}

fn main()
{

    let s = String::from("67772345234");

    println!("ans: {:?}", Solution::largest_good_integer(s));
}


