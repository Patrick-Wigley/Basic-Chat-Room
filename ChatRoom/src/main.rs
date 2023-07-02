use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: TcpStream) {
    println!("New connection");
    
}

fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:80");
    
    let listener:TcpListener;

    match listener_result {
        Ok(listener) => {
            println!("[SERVER]: Now Listening for connections");
            for incoming_stream in listener.incoming() {
                match incoming_stream {
                    Ok(s) => { handle_connection(s) },
                    Err(e) => { println!("[SERVER]: ERROR DURING CONNECTION - {}", e) }
                }
            }
        }
        Err(e) => {
            println!("[SERVER]: {}", e);
        }
    }

}
