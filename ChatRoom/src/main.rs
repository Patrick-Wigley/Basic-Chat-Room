use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read, Split}};
use std::time::Duration;

static mut players_details:Vec<String> = Vec::new();
const MAX_USERS:usize = 5;
static mut active_players_count:usize = 0;
// SUCCESS KEY = <:>


fn handle_connection(mut client: TcpStream) {
    let players_id:usize;
    unsafe {
        players_id = active_players_count;
        players_details[players_id] = format!("7,0;").to_string();
        active_players_count += 1;
    }
    println!("New connection");
    // This is a buffer for the bytes obtained/read throughout this stream
    let mut receive_data:[u8; 50] = [0u8; 50];
    
    loop {        
        let mut send_val:String;
        unsafe {
            send_val = stringvec_to_string(players_details.clone());
        }

        // Add error handle - (Result) for when client disconnects
        let _ = client.write(send_val.as_bytes());
       // println!("Sending: {}", send_val);
    
        // Read clients details
        let _ = client.read(&mut receive_data);
        let received_data_unpacked = std::str::from_utf8(&receive_data);
        
        match received_data_unpacked {
            Ok(msg) => {
                unsafe {
                    players_details[players_id] =  msg[0..(msg.find("~").unwrap())].to_string(); //[0..msg.len() - msg.find("~").unwrap()].to_string();
                    
                    println!("Players val: {:?}", players_details);
                }
            }
            Err(e) => {
                println!("ERROR: {}", e); break;
            }
        }
    }
    
}

fn main() {
    //For debbugi temp
    unsafe {
        players_details.resize(MAX_USERS, "1,0;".to_string());
    }
    
    
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
/// Converts Vector of player details to one string for transmission
fn stringvec_to_string(arr:Vec<String>) -> String {
   
    let mut ret:String = "#".to_string();
    for i in arr.iter() {
        // Add ID "{id}x,y;"
        ret.push_str(format!("{}", i).as_str());
  
    }

    ret
}

// fn f32vec_to_string(arr: Vec<[f32; 2]>) -> String
// {
//     let mut ret:String = "#".to_string();
    
//     for i in 0..arr.len() {
//         let values = arr[i];
//         ret.push_str(format!("{},{}; ", values[0], values[1]).as_str());
//     };

//     ret
// }