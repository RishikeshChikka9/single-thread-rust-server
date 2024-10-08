use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
     let mut buffer = [0; 1024];

     stream.read(&mut buffer).unwrap();

     let get = b"GET / HTTP/1.1\r\n";
     let style = b"GET /style.css HTTP/1.1\r\n";

     let (status_line, filename) =
         if buffer.starts_with(get){
             ("HTTP/1.1 200 OK", "../static_files/index.html")
         }else if buffer.starts_with(style){
             ("HTTP/1.1 200 OK", "../static_files/style.css")
         }else{
             ("HTTP/1.1 404 NOT FOUND", "../static_files/404.html")
         };

     let contents = fs::read_to_string(filename).unwrap();

     let response = format!(
         "{}\r\nContent-Length: {}\r\n\r\n{}",
         status_line,
         contents.len(),
         contents
         );

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap(); 
}
