




// hint3 : Use Manacher’s algorithm to compute the answer for each node in constant time.
// ‌Manacher算法‌是一种用于在线性时间内找到字符串中所有回文子串的算法

// g


// 现在 tle在 单链表上 ， 就是 10万个节点的 单链表。
// 有问题， 10万个结点，那么 调用栈要 10万层啊。这肯定不行的。  但没有报 栈溢出，都是 时间先超时
// 但是 单独拿出来 当作 Testcast跑，没有问题。 栈没有溢出。 rust会优化。？

// tle ... 数据结构应该没有问题了( dfsa3中不会申请新内存空间 )， 那就是算法错了。。
// tle
// tle
// tle

//   String 也不好用。。


// generate {parent -> children}
// postorder iter
// (char, index) -> [children]
// vvi
impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {

        let mut vvi = vec![vec![]; parent.len()];
        let mut vb = vec![true; parent.len()];

        for i in 1..parent.len() {  // [0] is root
            vvi[parent[i] as usize].push(i as i32);
        }

        // Self::dfsa1(0, &mut vb, &s, &vvi);
        // Self::dfsa2(0, &mut vb, &s.as_bytes().to_vec(), &vvi);

        let mut vu8 = vec![];
        vu8.reserve(parent.len());
        Self::dfsa3(0, &mut vb, &s.as_bytes().to_vec(), &vvi, &mut vu8, 0);

        vb
    }


    fn dfsa3(node: i32, vb: &mut Vec<bool>, s: &Vec<u8>, vvi: &Vec<Vec<i32>>, vu: &mut Vec<u8>, st: usize) {

        for i in 0..(vvi[node as usize]).len() {
            Self::dfsa3(vvi[node as usize][i], vb, s, vvi, vu, vu.len());
        }
        vu.push(s[node as usize]);

        let mut i = st;
        let mut j = vu.len() - 1;
        while i < j {
            if vu[i] != vu[j] {
                vb[node as usize] = false;
                break;
            }
            i += 1;
            j -= 1;
        }

        // for i in st..(vu.len() >> 1) {
        //     if vu[i] != vu[vu.len() - 1 - i] {
        //         vb[node as usize] = false;
        //         break;
        //     }
        // }

    }


    fn dfsa2(node: i32, vb: &mut Vec<bool>, s: &Vec<u8>, vvi: &Vec<Vec<i32>>) -> Vec<u8> {
        let mut vu = vec![];

        for i in 0..(vvi[node as usize]).len() {
            // let mut vu2 = Self::dfsa2(vvi[node as usize][i], vb, s, vvi);
            // vu.append(&mut vu2);
            vu.append(&mut Self::dfsa2(vvi[node as usize][i], vb, s, vvi));
        }
        vu.push(s[node as usize]);

        for i in 0..(vu.len() >> 1) {
            if vu[i] != vu[vu.len() - 1 - i] {
                vb[node as usize] = false;
                break;
            }
        }

        vu
    }

    fn dfsa1(node: i32, vb: &mut Vec<bool>, s: &String, vvi: &Vec<Vec<i32>>) -> String {
        let mut str = String::new();

        for i in 0..(vvi[node as usize]).len() {
            let str2 = Self::dfsa1(vvi[node as usize][i], vb, s, vvi);
            // str.push_str(str2.as_str());
            str += &str2;
        }
        // str.push(s.chars().nth(node as usize).unwrap());
        let mut str = str.as_bytes().to_vec();
        str.push(s.as_bytes()[node as usize]);
        let str = String::from_utf8(str).ok().unwrap();

        let str3 = str.as_bytes();
        for i in 0..(str.len()>>1) {
            if str3[i] != str3[str.len() - 1 - i] {
                vb[node as usize] = false;
                break;
            }
        }
        str
    }
}


struct Solution {}

fn main()
{
    let vi = [-1,0,0,1,1,2].to_vec();
    let s = "aababa".to_string();


    println!("ans: {:?}", Solution::find_answer(vi, s));
}


