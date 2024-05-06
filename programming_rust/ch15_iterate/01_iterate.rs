



// use std::string::String;

#![allow(dead_code, unused_imports)]



fn map_filter() {
    let text = "   ponies   \n    giraffes\niguanas\nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).filter(|s| *s != "iguanas").collect();
    
    println!("{:?}", v);

}


fn filter_map_flat_map()
{
    use std::str::FromStr;
    let text = "1\nfrontd .25    234\n3.14159 estauary.\n";
    
    // filter_map 迭代器会丢弃所有 None 值并为每个Some(v) 生成值 v
    for num in text.split_whitespace().filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", num.sqrt());
    }

// ---------------------

    use std::collections::HashMap;
    let mut cities = HashMap::new();
    cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    cities.insert("USA", vec!["Portland", "New York"]);
    cities.insert("Brazil", vec!["Sao Paulo", "Brasilis"]);
    let countries = ["Japan", "USA", /*"CN"*/];     // cities中不存在就报错
    for &city in countries.iter().flat_map(|country| &cities[country]) {
        println!("{}", city);
    }

}


fn flatten_()
{
    use std::collections::BTreeMap;

    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["park 1", "park 2"]);
    parks.insert("Kyoto", vec!["park 3", "park 4"]);
    parks.insert("Nas", vec!["park 5", "park 6"]);

    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    println!("{:?}", all_parks);

// --------------------

    let arr = vec![None, Some("day"), None, Some("one")];
    let vi = arr.into_iter().flatten().collect::<Vec<_>>();
    println!("{:?}", vi);

}


fn take_take_while()
{
    let msg = "To: anyone\r\n\
    from: supergo<editor@oreilly.com>\r\n\
    \r\n\
    Did you get any write today?\r\n\
    when will you stop waste?\r\n";

    for header in msg.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
}


fn skip_skip_while()
{
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
    }

    let msg = "To: anyone\r\n\
    from: supergo<editor@oreilly.com>\r\n\
    \r\n\
    Did you get any write today?\r\n\
    when will you stop waste?\r\n";

    for body in msg.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", body);
    }
}


use std::iter::Peekable;
fn parse_number<I>(tokens: &mut Peekable<I>) -> u32 where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}
fn peekable_()
{
    let mut chars = "123123,4234234".chars().peekable();
    println!("{}", parse_number(&mut chars));
    println!("{:?}", chars.next());
    println!("{}", parse_number(&mut chars));
    println!("{:?}", chars.next());
    println!("{:?}", chars.next());
}


fn double_end_iter_rev()
{
    let bee = ["head", "thorax", "abdomen"];
    let mut it = bee.iter();

    println!("{:?}", it.next());
    println!("{:?}", it.next_back());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next_back());

    let mut it2 = bee.iter().rev();
    // println!("{:?}", it.next());
    println!("{:?}", it2.next());   // <-
    println!("{:?}", it2.next_back());  // ->
    println!("{:?}", it2.next());
    println!("{:?}", it2.next_back());


}


fn chain_()
{
    // chain 迭代器会跟踪这两个底层迭代器是否返回了 None 并按需把其中一个迭代器的 next 和 next_back 重定向到另一个迭代器的next 和 next_back。
    let v : Vec<i32> = (1..5).chain([20,30,40]).collect();
    println!("{:?}", v);
}


fn enumerate_()
{
    let vi: Vec<i32> = vec![11,22,33,44,55];
    for (i, num) in vi.iter().enumerate() {
        println!("{:?}, {:?}", i, num);
    }
    println!("{:?}", vi);       // vi.into_iter，这里会panic
}


fn zip_()
{
    let v: Vec<_> = (0..).zip("ABCDE".chars()).collect();
    println!("{:?}", v);

    use std::iter::repeat;
    let vs = ["one", "two", "three"];
    let vzs: Vec<_> = repeat("going").zip(vs).collect();
    println!("{:?}", vzs);
}



fn by_ref_()
{

    let msg = "To: anyone\r\n\
    from: supergo<editor@oreilly.com>\r\n\
    \r\n\
    Did you get any write today?\r\n\
    when will you stop waste?\r\n";

    let mut lines = msg.lines();

    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nbody:");
    for body in lines {
        println!("{}", body);
    }

}


fn cloned_copied()
{
    let a = ['1','2','3','7'];
    println!("{:?}, {:?}", a.iter().next(), a.iter().cloned().next());
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().next(), Some('1').as_ref());
    assert_eq!(a.iter().cloned().next(), Some(&'1').copied());
    assert_eq!(a.iter().cloned().next(), Some('1'));
}


fn cycle_()
{
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();

    println!("{:?}, {:?}, {:?}", spin.next(), spin.next(), spin.next());
    println!("{:?}, {:?}, {:?}", spin.next(), spin.next(), spin.next());
    println!("{:?}, {:?}, {:?}", spin.next(), spin.next(), spin.next());

// -------------------

    use std::iter::{once, repeat};

    let fiz = repeat("").take(2).chain(once("fizz")).cycle();
    let buz = repeat("").take(4).chain(once("buzz")).cycle();
    let fiz_buz = fiz.zip(buz);

    let f_b = (1..20).zip(fiz_buz).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fiz, buz)) => format!("{}{}", fiz, buz),
    });

    for line in f_b {
        println!("{}", line);
    }
}


fn count_sum_product()
{
    use std::io::prelude::*;

    let stdin = std::io::stdin();   // linux: ctrl+d, windows: ctrl+z
    println!("{}", stdin.lock().lines().count());

    let mut t2: u64 = (1..=20).sum();
    println!("{}", t2);

    t2 = (1..=20).product();
    println!("{}", t2);


    // `std::iter::Sum` 特型和 `std::iter::Product` 特型
}


fn iter_min_max()
{
    let vi: Vec<i32> = vec![-2,1,2,4,-11,-3,5,-3];

    println!("{:?}, {:?}", vi.iter().max(), vi.iter().min());
    
    assert_eq!(vi.iter().max(), Some(&5));
    
}


use std::cmp::Ordering;
fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}
fn max_by_min_by()
{
    let num = [1.0, 4.0, 2.0];
    println!("{:?}, {:?}", num.iter().copied().max_by(cmp), num.iter().copied().min_by(cmp));

    // let num = [1.0, 2.0, std::f64::NAN];
    // println!("{:?}, {:?}", num.iter().copied().max_by(cmp), num.iter().copied().min_by(cmp));
                        // panic, Option.unwrap on the None

}


fn max_min_by_key()
{
    use std::collections::HashMap;

    let mut map2 = HashMap::new();
    map2.insert("Portland", 555_555);
    map2.insert("Fossil", 449);
    map2.insert("Greenhorn", 2);
    map2.insert("Boring", 7777);
    map2.insert("The Dalles", 15_555);

    println!("{:?}, {:?}", map2.iter().max_by_key(|&(_name, pop)| pop), map2.iter().min_by_key(|&(_name, pop)| pop));
}


fn any_all()
{
    let id = "Iterasd";
    println!("{:?}, {:?}", id.chars().any(char::is_uppercase), id.chars().all(char::is_uppercase));
}


fn position_rposition()
{
    let text = "Xerxes";
    println!("{:?}, {:?}", text.chars().position(|c| c == 'e'), text.chars().position(|c| c=='z'));

    let btx = b"Xerxes";
    println!("{:?}, {:?}", btx.iter().rposition(|&c| c == b'e'), btx.iter().rposition(|&c| c==b'X'));
}


fn fold_()
{
    let a = [4,5,6,7,8,9];

    println!("{:?}", a.iter().fold(0, |n, _| n+1)); // count
    println!("{:?}", a.iter().fold(0, |n, i| n+i)); // sum
    println!("{:?}", a.iter().fold(1, |n, i| n*i)); // production
    println!("{:?}", a.iter().cloned().fold(i32::min_value(), std::cmp::max));  // max


    let a = ["AAA", "BBB", "CCC", "DDD"];
    let t2 = a.iter().rfold(String::new(), |s, w| s + w + " ");
    println!("{}", t2);
}


use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;
fn fun2() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();           // linux:ctrl+d, win:ctrl+z
    let sum = stdin.lock().lines().try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
        Ok(sum + u64::from_str(&line?.trim())?)
    })?;
    println!("{}", sum);
    Ok(())
}

// fu fun4_all() // 可以用 try_fold 实现 all。

fn try_fold_try_rfold()
{
    println!(">>>> {:?} <<<<", fun2());
}



fn nth_nth_back()
{
    let mut sq = (0..10).map(|i| i*i);      // iterator

    println!("{:?}, {:?}, {:?}", sq.nth(4), sq.nth(0), sq.nth(7));  // 16, 25, None
}



fn find_rfind_find_map()
{

}



fn collect_()
{
    use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};
    let args: HashSet<String> = std::env::args().collect();
    println!("{:?}", args);
    let args: BTreeSet<String> = std::env::args().collect();
    println!("{:?}", args);
    let args: LinkedList<String> = std::env::args().collect();
    println!("{:?}", args);

    // 只有键–值对才能收集到Map中，因此对于这个例子，
    // 要把字符串序列和整数序列拉合在一起
    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    println!("{:?}", args);
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
    println!("{:?}", args);
}


fn extend_()    // std::iter::Extend
{
    let mut v : Vec<i32> = (0..5).map(|i| 1 << i).collect();
    v.extend([31,55,77,99]);
    println!("{:?}", v);
}




fn partition_()
{
    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];

    let (live, nolive) : (Vec<&str>, Vec<&str>) = things.iter().partition(|name| name.as_bytes()[0] & 1 != 0);

    println!("{:?}", live);
    println!("{:?}", nolive);
}



fn for_each_try_for_each()
{
    let vs = ["doves", "hens", "birds"];
    vs.iter().zip(["turtle", "frensh", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item,kind),quantity)| {
            format!("{} {} {}", quantity, kind, item)
        })
        .for_each(|gift| {
            println!("you receive: {}", gift);
        });
}





fn main() {

    // 需要 第三方的 rand 库
    // let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs())).take(100).collect();

    // use std::iter::FromIterator;
    // let mut outer = "Earth".to_string();
    // let inner = String::from_iter(outer.drain(1..4));
    // assert_eq!(outer, "Eh");
    // assert_eq!(inner, "art");

// -------------

    // map_filter();

    // filter_map_flat_map();

    // flatten_();

    // take_take_while();

    // skip_skip_while();

    // peekable_();

    // fuse // 一旦 返回 None， 后续必然 None

    // double_end_iter_rev();

    // inspect  // 为调试迭代器适配器的流水线提供了便利，但在生产代码中用得不多

    // chain_();

    // enumerate_();

    // zip_();

    // by_ref_();

    // cloned_copied();

    // cycle_();

// -------------- consume ---------------

    // count_sum_product();

    // iter_min_max();

    // max_by_min_by();

    // max_min_by_key();

    // eq_lt_ne_gt_ge_...   15.4.5  迭代器之间的比较

    // any_all();

    // position_rposition();

    // fold_();

    // try_fold_try_rfold();

    // nth_nth_back();

    // find_rfind_find_map();

    // collect_();
    
    // extend_();

    // partition_();

    for_each_try_for_each();



}