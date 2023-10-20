


// D D

// fn new(nestedList: Vec<NestedInteger>) -> Self {
//     fn flat(list: &Vec<NestedInteger>) -> Vec<i32> {
//         let mut ans = vec![];
//         for i in 0..list.len() {
//             match &list[i] {
//                 NestedInteger::Int(i) => ans.push(*i),
//                 NestedInteger::List(v) => {
//                     flat(&v).iter().for_each(|x| {
//                         ans.push(*x);
//                     });
//                 },
//             }
//         }
//         ans
//     }
//
//     let elements = flat(&nestedList);
//     let len = elements.len();
//     Self {
//         elements: elements,
//         position: 0,
//         len: len,
//     }
// }



// fn new(nestedList: Vec<NestedInteger>) -> Self {
//     let mut nums = vec![];
//     for elem in nestedList {
//         match elem {
//             Int(x) => nums.push(x),
//             List(list) => {
//                 let mut list = Self::new(list);
//                 while list.has_next() {
//                     nums.push(list.next());
//                 }
//             }
//         }
//     }
//     Self { index: 0, nums }
// }







// struct NestedIterator {
//     stack: Vec<NestedInteger>,
//     next: Option<i32>
// }

// impl NestedIterator {

//     fn new(mut nestedList: Vec<NestedInteger>) -> Self {
//         nestedList.reverse();
//         let mut res = NestedIterator {
//             stack: nestedList,
//             next: None,
//         };
//         res.next = res.get_next();
//         res
//     }
    
//     fn get_next(&mut self) -> Option<i32> {
//         match self.stack.pop() {
//             Some(NestedInteger::Int(n)) => Some(n),
//             Some(NestedInteger::List(mut v)) => {
//                 v.reverse();
//                 self.stack.extend(v);
//                 self.get_next()
//             },
//             _ => None,
//         }
//     }

//     fn next(&mut self) -> i32 {
//         let ret = self.next.unwrap();
//         self.next = self.get_next();
//         ret
//     }
    
//     fn has_next(&self) -> bool {
//         self.next.is_some()
//     }
// }



















// Runtime0 ms
// Beats
// 100%
// Memory3 MB
// Beats
// 57.14%

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    vn: Vec<i32>,
    idx: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut vn: Vec<i32> = Vec::new();
        Self::dfs_a1(&nestedList, &mut vn);
        NestedIterator {
            vn,
            idx: 0,
        }
    }
    
    fn next(&mut self) -> i32 {     // need mut.
        self.idx += 1;
        self.vn[self.idx - 1]
    }
    
    fn has_next(&self) -> bool {
        self.idx < self.vn.len()
    }

    fn dfs_a1(nlist: &Vec<NestedInteger>, vn: &mut Vec<i32>) {
        for ni in nlist.iter() {
            match ni {
                NestedInteger::Int(x) => vn.push(*x),
                NestedInteger::List(y) => Self::dfs_a1(y, vn),
                _ => (),
            }
        }
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
//



struct Solution {}

fn main()
{


    println!("ans: {:?}", Solution::());
}


