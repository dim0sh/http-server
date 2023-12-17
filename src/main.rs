use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;
fn handle_client(mut stream: TcpStream) {
    let buff = BufReader::new(&mut stream);
    let request_line = buff.lines().next().unwrap().unwrap(); 
    
    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /hello.html HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 200 OK", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {}", request_line);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    for stream in listener.incoming() {
        
        handle_client(stream?);
    }
    
    Ok(())
}
