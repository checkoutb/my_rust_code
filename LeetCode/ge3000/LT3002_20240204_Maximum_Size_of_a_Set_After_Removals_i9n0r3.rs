





// g


// error[E0502]: cannot borrow `set1` as mutable because it is also borrowed as immutable
//   --> ge3000/LT3002_20240204_Maximum_Size_of_a_Set_After_Removals_i9n0r3.rs:35:17
//    |
// 26 |             for i in &set1 {
//    |                      ----- immutable borrow occurs here
// ...
// 34 |             for i in &vi {
//    |                      --- immutable borrow later used here
// 35 |                 set1.remove(*i);
//    |                 ^^^^^^^^^^^^^^^ mutable borrow occurs here





impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

        unsafe {

        
        use std::collections::HashSet;

        // let mut set1 = HashSet::new();
        // let mut set2 = HashSet::new();

        let mut set1: HashSet<i32> = HashSet::new();
        let mut set2: HashSet<i32> = HashSet::new();

        for i in &nums1 {
            set1.insert(*i);            // ...
        }
        for i in &nums2 {
            set2.insert(*i);            // ...
        }

        let mut set11 = &set1;

        let mut sett = HashSet::new();

        if set11.len() > nums1.len() / 2 {
            let mut vi = vec![];
            unsafe {
                for i in &set1 {
                    if set2.contains(i) {
                        vi.push(i);
                        if set11.len() - vi.len() == nums1.len() / 2 {
                            break;
                        }
                    }
                }
            }
            unsafe {
                for i in vi {
                    // (&mut set11).remove(i);
                    sett.insert(*i);
                }
            }
        }
        let mut set1: HashSet<&i32> = set1.difference(&sett).collect::<HashSet<&i32>>();

        while set1.len() > nums1.len() / 2 {
            for i in &set1 {
                set1.remove(i);
                break;
            }
        }
        let mut set22 = &set2;
        sett.clear();
        if set22.len() > nums2.len() / 2 {
            let mut vi = vec![];
            unsafe {
                for i in &set2 {
                    if set1.contains(i) {
                        vi.push(i);
                        if set22.len() - vi.len() == nums2.len() / 2 {
                            break;
                        }
                    }
                }

                for i in vi.clone() {
                    // set2.remove(i);
                    sett.insert(*i);
                }
            }
        }
        let mut set2: HashSet<&i32> = set2.difference(&sett).collect::<HashSet<&i32>>();

        while set2.len() > nums2.len() / 2 {
            for i in &set2 {
                set2.remove(i);
                break;
            }
        }

        for i in set2 {
            set1.insert(i);
        }

        return set1.len() as i32;

        }
    }
}


struct Solution {}

fn main()
{

    let vi = [1,2,1,2].to_vec();
    let v2 = [1,1,1,1].to_vec();


    println!("ans: {:?}", Solution::maximum_set_size(vi, v2));
}


