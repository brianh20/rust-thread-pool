use crate::thread_pool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start_listener() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = thread_pool::ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
