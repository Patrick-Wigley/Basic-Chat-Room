use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read}};
use std::time::Duration;

static players_details:Vec<[f32;2]> = Vec::new();
const MAX_USERS:i32 = 5;

fn handle_connection(mut client: TcpStream) {
    println!("New connection");
    let mut data:&str = "Welcome to the server";
    //let _ = client.write(data.as_bytes());
    

    let mut val:[u32;2] = [0,10];

    loop {
        let temp = format!("user1:{:?}", val).to_string();
        data = temp.as_str();

        // Add error handle - (Result) for when client disconnects
        let _ = client.write(data.as_bytes());
        
    
        // Read clients details
        let mut receive_data:[u8; 128] = [0u8; 128];
        let _ = client.read(&mut receive_data);
        let received_data_unpacked = std::str::from_utf8(&receive_data);
        
        match received_data_unpacked {
            Ok(msg) => {
                println!("Received: {}", msg);
            }
            Err(e) => {
                println!("ERROR: {}", e); break;
            }
        }

        val[0] += 1;
        val[1] += 3;

        thread::sleep(Duration::from_millis(2));
    }
}

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
