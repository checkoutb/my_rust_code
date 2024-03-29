
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multi_thread_server::ThreadPool;


// https://course.rs/advance-practice1/multi-threads.html

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // one request, one thread
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     thread::spawn(|| {
    //         handle_connection(stream);
    //     });
    // }


    // thread pool
    let pool = ThreadPool::new(4);
    // for stream in listener.incoming() {

// take 是迭代器 Iterator 上的方法，会限制后续的迭代进行最多两次，然后就结束监听，
// 随后 ThreadPool 也将超出作用域并自动触发 drop。
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "resource/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "resource/hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "resource/404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}



