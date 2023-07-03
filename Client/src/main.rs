use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::{str, string};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:80");

    match stream {
        Ok(mut stream) => {
            println!("Connected to stream");

            let mut raw_data:[u8; 128] = [0u8; 128];
            loop {
                let _ = stream.read(&mut raw_data);
                let data = std::str::from_utf8(&raw_data);
                
                
                match data {
                    Ok(msg) => {
                        println!("{}", msg);
                    }
                    Err(e) => {println!("ERROR: {}", e); break;}
                }

            }
        }
        Err(e) => {println!("{}", e)}
    }

}
