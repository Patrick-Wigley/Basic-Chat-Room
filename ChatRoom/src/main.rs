use std::{net::{TcpListener, TcpStream}, thread};
use std::time::Duration;

fn handle_connection(stream: TcpStream) {
    println!("New connection");
        
}
const MAX_USERS:i32 = 5;

fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:80");

   
    match listener_result {
        Ok(listener) => {
            println!("[SERVER]: Now Listening for connections");
            for incoming_stream in listener.incoming() {
                match incoming_stream {
                    Ok(s) => { 
                        thread::spawn(|| {
                            handle_connection(s);
                        });
                      

                    },
                    Err(e) => { println!("[SERVER]: ERROR DURING CONNECTION - {}", e) }
                }
            }
        }
        Err(e) => {
            println!("[SERVER]: {}", e);
        }
    }
}
