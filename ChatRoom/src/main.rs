use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read}};
use std::time::Duration;

fn handle_connection(mut client: TcpStream) {
    println!("New connection");
    let mut data:&str = "Welcome to the server";
    //let _ = client.write(data.as_bytes());
    

    let mut val:[u32;2] = [0,10];

    loop {
        let temp = format!("user1:{:?}", val).to_string();
        data = temp.as_str();
        println!("Sending: {}", data);

        // Add error handle - (Result) for when client disconnects
        let _ = client.write(data.as_bytes());
        
        val[0] += 1;
        val[1] += 3;

        thread::sleep(Duration::from_millis(2));
    }
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
