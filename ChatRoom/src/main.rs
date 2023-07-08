use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read, Split}};
use std::time::Duration;

#[derive(Clone, Debug)]
struct PlayerDetails {
    position: String,
    messages: Vec<String>
}

const DEBUG:bool = false;

static mut PLAYERS_DETAILS:Vec<PlayerDetails> = Vec::new();
const MAX_USERS:usize = 5;
static mut ACTIVE_PLAYERS_COUNT:usize = 0;


// KEY:
// | - Player spot EMPTY
// ~ - End of players details
// ; - END of players string sent over through stream


fn handle_connection(mut client: TcpStream) {
    let mut players_id:usize = usize::MAX;
    unsafe {
        for (index, val) in PLAYERS_DETAILS.iter().enumerate(){
            if val.position == "|;" {
                // Spot Empty
                // Bring index back down to ZERO range
                players_id = index;
            }
        }
        if players_id == usize::MAX {
            panic!("[SERVER ERROR]: Cannot find empty spot for player. One of the previous player 
                strings may have become corrupt? - {:?}", PLAYERS_DETAILS); 
        }
        PLAYERS_DETAILS[players_id].position = format!("7,0;").to_string();
        ACTIVE_PLAYERS_COUNT += 1;
    }
    println!("[SERVER]: New connection: {}", players_id);
    // This is a buffer for the bytes obtained/read throughout this stream
    let mut receive_data:[u8; 50] = [0u8; 50];
    
    loop {        
        let send_val:String;
        unsafe {
            let mut other_players_details:Vec<PlayerDetails> = PLAYERS_DETAILS.clone();
            other_players_details.remove(players_id);
            send_val = stringvec_to_string(other_players_details);
        }
        
      //  println!("Players val: {:?}", send_val);
        // Add error handle - (Result) for when client disconnects
        let write_result = client.write(send_val.as_bytes());
        match write_result {
            Ok(_r) => {}
            Err(e) => {
                // Connection is no longer existent - (Local may have abruptly lost connection or forcibly left)
                if DEBUG {println!("[SERVER ERROR]: {}", e);}
                handle_disconnect(players_id);
                break;
            }
        }

        // Read clients details
        let _ = client.read(&mut receive_data);
        let received_data_unpacked = std::str::from_utf8(&receive_data);
        
        match received_data_unpacked {
            Ok(msg) => {
                if msg.contains("(DISCONNECT)") {
                    // Player is disconnecting, handle this and make room for other connections to take this place
                    handle_disconnect(players_id);
                    break;
                }

                unsafe {
                    PLAYERS_DETAILS[players_id].position = msg[0..(msg.find("~").unwrap())].to_string(); //[0..msg.len() - msg.find("~").unwrap()].to_string();
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
        PLAYERS_DETAILS.resize(MAX_USERS, PlayerDetails { position: ("|;".to_string()), messages: (Vec::new()) });
    }
        
    let listener_result = TcpListener::bind("127.0.0.1:80");
    

    match listener_result {
        Ok(listener) => {
            println!("[SERVER]: Now Listening for connections");
            for incoming_stream in listener.incoming() {
                match incoming_stream {
                    Ok(s) => { 
                        let current_active_players_count: usize;
                        let current_players_details: Vec<PlayerDetails>;
                        unsafe {
                            current_active_players_count = ACTIVE_PLAYERS_COUNT.clone();
                            current_players_details = PLAYERS_DETAILS.clone();
                        }
                        if current_active_players_count >= current_players_details.len() {
                            println!("Client attempted to join but lobby has reached maximum limit");
                        } else {
                            // Add new connection as there's available space
                            thread::spawn(|| {
                                handle_connection(s);
                            });
                        }

                      
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
fn stringvec_to_string(arr:Vec<PlayerDetails>) -> String {
    let mut ret:String = "#".to_string();
    for player in arr.iter() {
        // Add ID "{id}x,y;"
        ret.push_str(format!("{}", player.position).as_str());
    }
    ret
}
/// Executes on disconnection of a client
fn handle_disconnect(player_id:usize) {
    println!("[SERVER]: Connection lost: {}", player_id);
    unsafe {
        ACTIVE_PLAYERS_COUNT -= 1;
        PLAYERS_DETAILS[player_id].position = "|;".to_string();
    };
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