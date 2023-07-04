use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::{str, string, thread};

use tetra::{graphics::{mesh::{GeometryBuilder, Mesh, ShapeStyle}, self, Color}, input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::Vec2;

struct GameState {
    local_player_position: [f32; 2],

    global_player_positions: Vec<[f32; 2]>,
    
    player_shape: Mesh
}
static players_details:Vec<[f32;2]> = Vec::new();
const STARTING_POSITION:[f32; 2] = [0.0, 0.0];
const CURRENT_PLAYER:usize = 0; 
const MOVEMENT_SPEED: f32 = 2.0;

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {  
            local_player_position: [0.0; 2],
            global_player_positions: Vec::new(),
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?
        })   
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player_shape.draw(ctx, Vec2::from(self.local_player_position));

        Ok(())
    }  
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut speed:f32 = 0.0;
        if input::is_key_down(ctx, Key::LeftShift) {
            speed = 2.0;
        }
        if input::is_key_down(ctx, Key::W) {
            self.local_player_position[1] -= MOVEMENT_SPEED + speed;
        }
        else if input::is_key_down(ctx, Key::S) {
            self.local_player_position[1] += MOVEMENT_SPEED + speed;
        }
        if input::is_key_down(ctx, Key::A) {
            self.local_player_position[0] -= MOVEMENT_SPEED + speed;
        }
        else if input::is_key_down(ctx, Key::D) {
            self.local_player_position[0] += MOVEMENT_SPEED + speed;
        }

        Ok(())
    }
}


fn setup_window() -> tetra::Result {
      ContextBuilder::new("Online Squares!", 1280, 720)
        .build()?
        .run(GameState::new)
}
fn main() {
    thread::spawn(server_handle);
    
    let _ = setup_window();
    
}


// Server Communications
fn server_handle() {
    let stream = TcpStream::connect("127.0.0.1:80");

    match stream {
        Ok(mut stream) => {
            println!("Connected to server");

            let mut raw_data:[u8; 128] = [0u8; 128];
            loop {
                let _ = stream.read(&mut raw_data);
                let data = std::str::from_utf8(&raw_data);
                
                match data {
                    Ok(msg) => {
                        println!("Server says: {}", msg);
                    }
                    Err(e) => {println!("ERROR: {}", e); break;}
                }
                
                let _ = stream.write("Hi Server".as_bytes());
            }
        }
        Err(e) => { panic!("{}", e) }
    }
}