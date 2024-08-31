use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = 
        match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 Ok", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(20));
                ("HTTP/1.1 200 OK", "hello.html")
            },
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
    let content = fs::read_to_string(file_name).unwrap();
    let content_length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
    
}