use std::net::{SocketAddr, TcpStream};

fn main() {
    
    let stream = TcpStream::connect("127.0.0.1:80");

    match stream {
        Ok(r) => {
            println!("Connected to server");
        }
        Err(e) => {println!("{}", e)}
    }

}
