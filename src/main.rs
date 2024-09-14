use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, html_page) = if buffer.starts_with(get) {
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(html_page).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// HTTP-version Status-Code Reson-Phrase CRLF
// Headers CRLF
// message-body
//
// ex: HTTP/1.1 200 OK\r\n\r\n
