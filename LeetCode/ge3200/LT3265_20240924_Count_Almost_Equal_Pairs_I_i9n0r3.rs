


// D 

// 把 int 转为 string，并且 添加前置0，使得 string 的size 是 6
// for for  交换 string 的 2个char，  查看 map中是否有，  然后加入到 map

// .................



// g


// 100, brute-force

// 3 30
// 333   3000330

impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0i32;

        for i in 0..nums.len() {
            for j in i+1 .. nums.len() {
                if Solution::is_equal(nums[i], nums[j]) {

                    println!("{}, {}", nums[i], nums[j]);

                    ans += 1;
                } else {
                    // println!("nnnno {}, {}", nums[i], nums[j]);
                }
            }
        }

        ans
    }

    fn is_equal(a: i32, b: i32) -> bool {
        if a == b {
            return true;
        }

        let sa2 = a.to_string();
        let sb2 = b.to_string();

        let mut sa = sa2.as_bytes();
        let mut sb = sb2.as_bytes();

        if sa.len() == sb.len() {
            let mut cnt = 0;
            let mut lst = 0;
            // let mut used = false;

            for i in 0..sa.len() {
                if sa[i] != sb[i] {
                    if cnt == 0 {
                        lst = i;
                        cnt += 1;
                    } else if cnt == 1 {
                        if sa[lst] == sb[i] && sa[i] == sb[lst] {
                            // used = true;
                        } else {
                            return false;
                        }
                        cnt += 1;
                    } else {
                        return false;
                    }
                }
            }
            if cnt == 1 {  // ...
                return false;
            }
        } else {
            if sa.len() < sb.len() {
                (sa,sb) = (sb,sa);
            }
            // sa.len > sb.len

            // if sa[1] != b'0' { // ....
            //     return false;
            // }

            let cha = sa[0];

            let mut idx = 1usize;
            while idx < sa.len() && sa[idx] == b'0' {
                idx += 1;
            }

            let mut used = false;
            for i in 0..sb.len() {
                if idx == sa.len() || sb[i] != sa[idx] {
                    if sb[i] != cha {

                        // println!("111 {}, {}   {}, {:?}, {}, {:?}", i, sb[i], cha, sb, idx, sa);
                        return false;
                    }
                    if idx < sa.len() && sa[idx] != b'0' {  // g
                        return false;
                    }
                    if idx < sa.len() {
                        idx += 1;
                    }
                    if used {
                        // println!("222");
                        return false;
                    }
                    used = true;
                } else {
                    idx += 1;
                }
            }
            if idx != sa.len() {
                // println!("333");
                return false;
            }
        }
        true
    }
}


impl Solution {
    fn fun1(vi: Vec<i32>) {
        let sz1 = vi.len();
        let mut v2 = vec![0; sz1];

        for idx in 0..sz1 {
            let mut v3 = vec![0; 10];
            let mut t2 = vi[idx];
            while t2 > 0 {
                v3[(t2 % 10) as usize] += 1;
                t2 /= 10;
            }
            t2 = 0;
            for i in 1..10 {
                while v3[i] > 0 {
                    v3[i] -= 1;
                    t2 = t2 * 10 + i as i32;
                }
            }
            v2[idx] = t2;
        }

        println!("{:?}", v2);
        let mut cnt = 0;
        for i in 0..sz1 {
            for j in i+1 .. sz1 {
                if v2[i] == v2[j] {
                    println!("{}, {}, {}", v2[i], vi[i], vi[j]);
                    cnt += 1;
                }
            }
        }
        println!(" total equal: {}", cnt);
    }
}

struct Solution {}

fn main()
{

    let vi = [3,12,30,16,21,3133,3000133].to_vec();
    // let vi = [8,12,5,5,14,3,12,3,3,6,8,20,14,3,8].to_vec();

    // 28
    // let vi = [490693,900498,448195,24359,126032,584252,26132,124479,586672,855404,24359,418495,243450,106232,690685,410981,871863,419180,242524,23549,284759,26132,271146,966337,781863,418495,242524,126032,411980,621032,271641,25349,900894,411980,997268,671059,649498,781836,312273,15727,671095].to_vec();

    // let vi = [126032,26132].to_vec();

    println!("ans: {:?}", Solution::count_pairs(vi));


    // let v2 = [490693,900498,448195,24359,126032,584252,26132,124479,586672,855404,24359,418495,243450,106232,690685,410981,871863,419180,242524,23549,284759,26132,271146,966337,781863,418495,242524,126032,411980,621032,271641,25349,900894,411980,997268,671059,649498,781836,312273,15727,671095].to_vec();
    // Solution::fun1(v2);
}


// 900498, 900894
// 448195, 418495
// 448195, 418495
// 24359, 24359
// 24359, 25349
// 126032, 106232
// 126032, 126032
// 126032, 621032
// 26132, 26132
// 24359, 25349
// 418495, 418495
// 106232, 126032
// 410981, 411980
// 410981, 411980
// 871863, 781863
// 419180, 411980
// 419180, 411980
// 242524, 242524
// 23549, 25349
// 271146, 271641
// 781863, 781836
// 126032, 621032
// 411980, 411980
// 671059, 671095
// ans: 24

// 4899, 900498, 900894
// 144589, 448195, 418495
// 144589, 448195, 418495
// 23459, 24359, 24359
// 23459, 24359, 23549.........
// 23459, 24359, 25349
// 12236, 126032, 26132     !!!
// 12236, 126032, 106232
// 12236, 126032, 26132
// 12236, 126032, 126032
// 12236, 126032, 621032
// 12236, 26132, 106232
// 12236, 26132, 26132
// 12236, 26132, 126032
// 12236, 26132, 621032
// 23459, 24359, 23549
// 23459, 24359, 25349
// 144589, 418495, 418495
// 12236, 106232, 26132
// 12236, 106232, 126032
// 12236, 106232, 621032
// 11489, 410981, 419180
// 11489, 410981, 411980
// 11489, 410981, 411980
// 136788, 871863, 781863
// 136788, 871863, 781836
// 11489, 419180, 411980
// 11489, 419180, 411980
// 222445, 242524, 242524
// 23459, 23549, 25349
// 12236, 26132, 126032
// 12236, 26132, 621032
// 112467, 271146, 271641
// 136788, 781863, 781836
// 12236, 126032, 621032
// 11489, 411980, 411980
// 15679, 671059, 671095
//  total equal: 37