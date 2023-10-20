fn main() {
    
    let x = (32.01_f32).sqrt();
    if x.is_nan() {
        
    } else {
        println!("{}", x);
    }

    let a = 22i32;
    let b = 1_000_000;
    let arr = [32.0, 22f32, 22.0_f32];

    println!("{},{},{:?}", a, b, arr);

    // --------------------
    let x = {
        let a = 123;
        a * 2
    };        // ;;;;;;;;
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };       // ;;;;;;;;;;;;;
    let z = if x % 2 == 1 {"odd222"} else {"even222"};

    println!("{},{},{}", x, y, z);


    // ------------------
    let s1 = String::from("hi");
    let s2 = s1;         // 所有权发生了转移
    // println!("{}", s1);      // 编译失败。value used here after move

    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);      // ok

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);




    let s = String::from("hello world");
    let hello = &s[0..5];       // slice        右半开区间
    let world = &s[6..11];
    
    let slice = &s[0..2];
    let slice = &s[..2];        // 等价于上面

    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..];        // 等价于上面

    let slice = &s[0..len];
    let slice = &s[..];     // 等价于上面


    // let s = "中国人";
    // let a = &s[0..2];    // 切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界
    // println!("{}",a);    // error


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    //该数组切片的类型是 &[i32]


    let s = String::from("hello,world");    // &str -> String
    let s = "hello,world".to_string();

    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);      // String -> &str
    say_hello(s.as_str());


    let tup = (222,123.2,1);
    let (x,y,z) = tup;
    println!("{}", y);

    let tup: (i32,f64,u8) = (111,222.2,3);
    println!("{}", tup.1);

}
fn say_hello(s: &str) {
    println!("{}",s);
}
fn clear(text: &mut String) -> () {
    *text = String::from("");
}
fn clear22(mut text: String) -> () {
    text = String::from("");
}
// fn clear2(mut text: &String) -> () {     // ok
//     *text = String::from("");        // `text` is a `&` reference, so the data it refers to cannot be written
// }

// fn clear3(&mut text: String) -> () {     // expected `String`, found `&mut _`
//     *text = String::from("");
// }


// owner
fn main2() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作



fn main3() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}




// #![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main4() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     //read(&mut f1, &mut vec![]);
//     close(&mut f1);
// }



// 借用的规则：当我们已经有了可变借用时，就无法再拥有不可变的借用。
// 因为 clear 需要清空改变 String，因此它需要一个可变借用
// （利用 VSCode 可以看到该方法的声明是 pub fn clear(&mut self) ，参数是对自身的可变借用 ）；
// 而之后的 println! 又使用了不可变借用，
// 也就是在 s.clear() 处可变借用与不可变借用试图同时生效，因此编译无法通过。
// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }