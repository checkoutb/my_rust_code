
use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("app error: {e}");
        process::exit(1);
    }
}


// better
// --------- iter st
// fn main() {
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     // --snip--
// }



// // index will array out of bound
// // so use iter is better
// impl Config {
//     pub fn build(
//         mut args: impl Iterator<Item = String>,
//     ) -> Result<Config, &'static str> {
//         // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
//         args.next();

//         let query = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),
//         };

//         let file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file path"),
//         };

//         let ignore_case = env::var("IGNORE_CASE").is_ok();

//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }




// // in lib.rs
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }


// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }
// --------- iter en




// use std::env;
// use std::fs;

// fn main() {
//     // mainv1();
//     mainv2();
// }

// // -------------- v1 start
// fn mainv1() {
//     // println!("Hello, world!");

//     let args: Vec<String> = env::args().collect();
//     // dbg!(args);      // owner move .. &args[1] will compile error

//     let query = &args[1];
//     let file_path = &args[2];

//     let contents = fs::read_to_string(file_path).expect("should have beed able to read file");

//     println!("with text:\n{contents}");
// }
// // --------------- v1 end


// // ------------- v2 start
// fn mainv2() {
//     let args: Vec<String> = env::args().collect();
//     let (query, file_path) = parse_config_v2(&args);

// }

// fn parse_config_v2(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }
// // -------------- v2 end


// // ---------- v3 st
// fn mainv3() {

// }
// struct Config {
//     query: String,
//     file_path: String,
// }
// fn parse_config_v3(args: &[String]) ->Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config {query, file_path}
// }
// // -------------- v3 en


// // ------------- v4 st
// fn mainv4() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::new(&args);
// }
// impl Config {       // v3's config
//     fn new(args: &[String]) -> Config {
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config {query, file_path}
//     }
// }
// // --------------- v4 en

// // --------------- v5 st, v5 == v4 + error handler
// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         // ...snip
//     }
// }
// // --------------- v5 end

// // -------------- v6 st
// // panic will show: rust file name, line, column ... user don't know what them are.
// // so use Result replace panic
// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config{query, file_path})
//     }
// }

// fn mainv6() {
//     use std::process;

//     let args: Vec<String> = env::args().collect();
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("problem parsing arguments: {err}");
//         process::exit(1);
//     });
//     println!("search for {}", config.query);
//     println!("in file {}", config.file_path);

//     let contents = fs::read_to_string(config.file_path).expect("should have been able to read the file");
//     println!("with text:\n{contents}");
// }
// // -------------- v6 en


// // -------------- v7 st
// use std::process;
// fn mainv7() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);

//     run(config);
// }

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }
// // -------------- v7 en

// // -------------- v8 st
// use std::error::Error;
// fn runv8(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("with text:\n{contents}");
//     Ok(())
// }

// fn mainv8() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);

//     if let Err(e) = run(config) {
//         println!("app error: {e}");
//         process::exit(1);
//     }
// }
// // -------------- v8 en
