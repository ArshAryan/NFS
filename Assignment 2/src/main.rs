use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => {
            send_response(&mut stream, "HTTP/1.1 500 INTERNAL SERVER ERROR", "500.html");
            return;
        }
    };

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_line.starts_with("GET ") {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    } else {
        ("HTTP/1.1 400 BAD REQUEST", "400.html")
    };

    send_response(&mut stream, status_line, filename);
}

fn send_response(stream: &mut TcpStream, status_line: &str, filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            let error_response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nInternal Server Error";
            stream.write_all(error_response.as_bytes()).unwrap();
            return;
        }
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
