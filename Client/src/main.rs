use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;

use tetra::{graphics::{mesh::{Mesh, ShapeStyle}, self, Color}, input::{self, Key}, Context, ContextBuilder, State};
use tetra::math::Vec2;


/* GLOBALS & CONSTS */
static mut LOCAL_DETAILS:[f32; 2] = [0.0, 0.0];
static mut PLAYERS_DETAILS:Vec<[f32;2]> = Vec::new();
const MOVEMENT_SPEED: f32 = 2.0;

/* TETRA */
struct GameState {
    local_player_position: [f32; 2],  
    player_shape: Mesh,    
}
impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {  
            local_player_position: [0.0; 2],
            player_shape: Mesh::circle(ctx, ShapeStyle::Stroke(10.0), Vec2::zero(), 10.0)?,
        })   
    }
}
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // Draw Local Player
        self.player_shape.draw(ctx, Vec2::from(self.local_player_position));

        // Draw Other Players
        unsafe {
            for i in PLAYERS_DETAILS.iter() {
                self.player_shape.draw(ctx, Vec2::from(i.clone()));
            }
        }
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

        // Setting global var to be used in 'server_handle' thread
        unsafe {
            LOCAL_DETAILS = self.local_player_position.clone();
        }

        Ok(())
    }
}
/// Returns built tetra application
fn setup_window() -> tetra::Result {
    ContextBuilder::new("Online Squares!", 1280, 720)
        .build()?
        .run(GameState::new)    
}

fn main() {
    thread::spawn(server_handle);
    let _ = setup_window();    
}

/* SERVER COMMUNICATIONS */
/// TCP Stream for current client & server communications of data \ 
/// Handled by its own thread
fn server_handle() {
    let stream = TcpStream::connect("127.0.0.1:80");
    
    match stream {
        Ok(mut stream) => {
            let mut raw_receive_data:[u8; 128] = [0u8; 128];
            
            loop {
                /* RECEIVING DATA FROM SERVER */               
                let _ = stream.read(&mut raw_receive_data);
                let data = std::str::from_utf8(&raw_receive_data);
                match data {
                    Ok(msg) => {
                        unsafe {
                            PLAYERS_DETAILS = get_players_from_string(String::from(msg));
                        }
                    }
                    Err(e) => {println!("[ERROR]: {}", e);}
                }

                /* SENDING LOCAL DETAILS TO SERVER - (X,Y Position) */
                let send_val:String;
                unsafe {
                    send_val = get_string_from_local_details(LOCAL_DETAILS.clone());
                }
                let _ = stream.write(send_val.as_bytes());
            }
        }
        Err(e) => { 
            panic!("[ERROR]: {}", e)
        }
    }
}

/* SUB FUNCTIONS USED FOR 'server_handle' */
/// Prepares string which will be sent over to the server \
/// Returns prepared String
fn get_string_from_local_details(arr: [f32;2]) -> String {
    format!("{},{};~ ", arr[0], arr[1])      
}
/// Gathers each players details from string received & processes it to Vec<[f32; 2]> \
/// Returns processed Vector
fn get_players_from_string(data: String) -> Vec<[f32; 2]> {
    let mut ret:Vec<[f32; 2]> = Vec::new();

    ret.resize(data.matches(";").count(), [0.0; 2]);

    let mut player_index:usize = 0;

    let mut xy_index = 0;
    
    let mut xy_str:[String; 2] = ["".to_string(), "".to_string()];

    for char in data.chars() {
        if char == ',' {
            // Seperation of 2 values (X,Y)
            xy_index = 1;
        }
        else if char == ';' {
            // At this point, 'xy_str' has collected the players details
            let mut xy_f32 = [0.0, 0.0];
            for i in 0..2 {
                let val = xy_str[i].parse::<f32>();
                xy_str[i] = "".to_string();                                     // Clear contents out for next player iteration
                match val {
                    Ok(v) => { xy_f32[i] = v; }
                    Err(_e) => { 
                        // End of string seems to be corrupted sometimes? - Returns the processed data
                        return ret;
                    }
                }
            }
            ret[player_index] = xy_f32;
            xy_index = 0; 
            player_index += 1;
        }
        else if char.is_numeric() || char == '.' || char == '-'{
            xy_str[xy_index].push(char);
        }
    }
    ret
}