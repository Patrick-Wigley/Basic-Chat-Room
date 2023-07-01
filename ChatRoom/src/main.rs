use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) {
    println!("New connection");
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    
    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(s) => {
                handle_connection(s)
            }
            Err(e) => {  println!("ERROR DURING CONNECTION")   }
        }
    }

    println!("Hello, world!");
}
