// use std::net::TcpListener;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     println!("Connection established!");
    // }
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);
    }
}

// fn handle_connection(mut stream: &TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     // println!("Request: {http_request:#?}")
//     // let response = "HTTP/1.1 200 OK\r\n\r\n";
//     // stream.write_all(response.as_bytes()).unwrap();

//     let status_line = "HTTP/1.1 200 OK";
//     let contents = fs::read_to_string("hello.html").unwrap();
//     let content_length = contents.len();
//     let response = format!("{status_line}\r\nContent-Length: {content_length}\r\nCache-Control: no-cache\r\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

// 验证请求并有选择的进行响应
fn handle_connection(mut stream: &TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // 优化
    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    // 在当前 server 实现中模拟慢请求
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
