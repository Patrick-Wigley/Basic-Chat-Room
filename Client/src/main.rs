use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::{str, string};

fn main() {
    
    let mut stream = TcpStream::connect("127.0.0.1:80");

    match stream {
        Ok(mut server) => {
            println!("Connected to server");

            let mut raw_data = [0u8; 128];
            server.read(&mut raw_data);
            
            let data = std::str::from_utf8(&raw_data);
            

            match data {
                Ok(msg) => {
                    println!("{}", msg);
                }
                Err(e) => {println!("ERROR: {}", e)}
            }
        }
        Err(e) => {println!("{}", e)}
    }

}
