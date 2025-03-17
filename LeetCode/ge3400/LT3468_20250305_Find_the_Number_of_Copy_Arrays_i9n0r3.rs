




// D D

        // for i in 0..original.len() {
        //     offset_min = offset_min.max(bounds[i][0] - original[i]);
        //     offset_max = offset_max.min(bounds[i][1] - original[i]);
        // }



// Runtime
// 6ms
// Beats40.74%
// Memory
// 10.43MB
// Beats81.48%





// 差要相同。
// 1 - 100...000

impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let mut st = bounds[0][0];
        let mut en = bounds[0][1];
        let mut diff = 0;
        for i in 1..bounds.len() {
            if st > en {
                break;
            }
            diff += original[i] - original[i - 1];
            if st + diff < bounds[i][0] {    // want st + diff == bounds[i][0]
                st = bounds[i][0] - diff;
            }
            if en + diff > bounds[i][1] {
                en = bounds[i][1] - diff;
            }
        }

        (en - st + 1).max(0)
    }
}


struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


