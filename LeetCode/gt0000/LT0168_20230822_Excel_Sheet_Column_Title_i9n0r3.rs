




// g
// column_number-- at every loop


/*
1 - A       26*0 + 1
26 - Z

AA - 27         26*1 + 1
AZ - 26*2       26*1 + 26

BA - 26*2+1     26*2 + 1
BZ              26*2 + 26

ZZ -            26*26 + 26          26*27
AAA         26*27 + 1
AAZ     26*27+26

ABA         26*28 + 1

ZZZ  27 * (27 * (26))

*/
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut vi = vec![26 as i64; 1];
        let mut column_number = column_number as i64;
        let mut t2 = 26 as i64;
        while column_number >= t2 {
            t2 *= 27;
            vi.push(t2);
        }
        let mut ans = "".to_string();
        vi.reverse();

        println!("{:?}", vi);

        for i in 0..vi.len() {

            
            if column_number as i64 > vi[i] {
                println!(" - {:?}", column_number);
                ans.push(Self::to_char(column_number / vi[i]));
                column_number %= vi[i];
            }
        }
        ans.push(Self::to_char(column_number));
        ans
    }

    pub fn to_char(t2 : i64) -> char {
        // println!("{:?}", t2 as u32);
        // println!("-- {:?}", 'A' as u32);
        let c = char::from_u32(t2 as u32 + ('A' as u32) - 1);
        c.unwrap()
    }
}



struct Solution {}

fn main()
{

    // AQE
    println!("ans: {:?}", Solution::convert_to_title(421));
}


