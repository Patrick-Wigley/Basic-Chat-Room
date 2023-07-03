use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::{str, string};

use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}


fn setup_window() -> tetra::Result {
      ContextBuilder::new("Online Squares!", 1280, 720)
        .build()?
        .run(|_| Ok(GameState))
}
fn main() {
    let stream = TcpStream::connect("127.0.0.1:80");


    let _ = setup_window();


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
        Err(e) => {println!("Could not connect: {}", e)}
    }

}
