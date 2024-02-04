
// use std::net::TcpListener;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 当 stream 超出作用域时，会触发 drop 的扫尾工作，其中包含了关闭连接。
    // 但是，浏览器可能会存在自动重试的情况，因此还会重新建立连接，最终打印了多次。
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("connected established!");
        handle_connection_v2(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // // 浏览器空白页面
    // // 不加response的话，浏览器是错误页面 The connection was reset
    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();


    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("resource/hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    println!("request: {:#?}", http_request);
}

fn handle_connection_v2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("resource/hello.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("resource/404.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "resource/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "resource/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}


